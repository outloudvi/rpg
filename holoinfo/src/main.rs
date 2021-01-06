use colored::*;
use phf::phf_map;
use reqwest::Url;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Channel {
    vtuber_id: String,
    view_count: i32,
    subscriber_count: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
struct Good {
    updated_at: String,
    channels: Vec<Channel>,
}
#[derive(Serialize, Deserialize, Debug)]
struct Bad {
    code: u32,
    message: String,
}
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum ReqResult {
    Err(Bad),
    Ok(Good),
}

static POI_ENDPOINT: &str = "https://holo.poi.cat/api/v3/youtube_channels";

static AVAILABLE_VTUBERS: phf::Map<&'static str, &'static str> = phf_map! {
    "hololive" => "Hololive",
    "sora" => "时乃空",
    "roboco" => "萝卜子",
    "miko" => "樱巫女",
    "suisei" => "星街彗星",
    "fubuki" => "白上吹雪",
    "matsuri" => "夏色祭",
    "haato" => "赤井心",
    "aki" => "亚绮罗森塔尔",
    "mel" => "夜空梅露",
    "choco" => "愈月巧可",
    "choco_alt" => "愈月巧可 (2)",
    "shion" => "紫咲诗音",
    "aqua" => "凑阿夸",
    "subaru" => "大空昴",
    "ayame" => "百鬼绫目",
    "pekora" => "兔田佩克拉",
    "rushia" => "润羽露西娅",
    "flare" => "不知火芙蕾雅",
    "marine" => "宝钟玛琳",
    "noel" => "白银诺艾尔",
    "kanata" => "天音彼方",
    "coco" => "桐生可可",
    "watame" => "角卷绵芽",
    "towa" => "常暗永远",
    "himemoriluna" => "姬森璐娜",
    "lamy" => "雪花菈米",
    "nene" => "nene",
    "botan" => "狮白牡丹",
    "polka" => "尾丸波尔卡",
    "mio" => "大神澪",
    "okayu" => "猫又小粥",
    "korone" => "戌神沁音",
    "azki" => "AZKi",
    "risu" => "Ayunda Risu",
    "moona" => "Moona Hoshinova",
    "iofi" => "Airani Iofifteen",
    "ollie" => "Kureiji Ollie",
    "melfissa" => "Anya Melfissa",
    "reine" => "Pavolia Reine",
    "amelia" => "华生阿米莉亚",
    "calliope" => "森美声",
    "gura" => "噶呜古拉",
    "inanis" => "一伊那尓栖",
    "kiara" => "小鸟游琪亚拉",
    "nana" => "神乐七奈",
    "ui" => "时雨羽衣",
};

fn create_url(vt: &Vec<&'static str>) -> Result<Url, ()> {
    let given_args: Vec<String> = std::env::args().skip(1).collect();
    let final_args: Vec<String> = if given_args.contains(&"*".to_string()) {
        print!("{}", "[WARN] ".yellow());
        println!("Selecting all VTubers.");
        vt.into_iter().map(|x| String::from(*x)).collect()
    } else {
        given_args
            .into_iter()
            .filter(|x| {
                if !vt.contains(&x.as_str()) {
                    print!("{}", "[WARN] ".yellow());
                    println!("{} is not found. Ignoring.", x);
                    false
                } else {
                    true
                }
            })
            .collect()
    };
    if final_args.len() == 0 {
        print!("{}", "[WARN] ".yellow());
        println!("No valid VTuber selected.");
        return Err(());
    }
    let mut url = Url::parse(POI_ENDPOINT).unwrap();
    url.query_pairs_mut()
        .clear()
        .append_pair("ids", &final_args.join(","));
    Ok(url)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vtuber_slugs: Vec<&'static str> = AVAILABLE_VTUBERS.keys().cloned().collect();

    let url = create_url(&vtuber_slugs);
    if let Err(_) = url {
        return Ok(());
    }
    let url = url.unwrap();
    let resp = reqwest::get(url.as_str()).await?;
    let text = resp.text().await?;
    // println!("{:?}", text);

    let obj = match serde_json::from_str(&text)? {
        ReqResult::Err(b) => Err(b),
        ReqResult::Ok(v) => Ok(v),
    };

    if let Err(e) = obj {
        print!("{}", "[ERR] ".red());
        println!("Error {}: {}", e.code, e.message);
        return Ok(());
    }
    let obj = obj.unwrap();
    println!("{}", "[INFO]".green());
    println!("Updated at: {}", obj.updated_at);
    let mut chan = obj.channels;
    chan.sort_by(|b, a| a.subscriber_count.partial_cmp(&b.subscriber_count).unwrap());
    println!("Name | View | Sub");
    for i in chan {
        println!(
            "{} | {} | {}",
            AVAILABLE_VTUBERS
                .get(i.vtuber_id.as_str())
                .cloned()
                .unwrap(),
            i.view_count,
            i.subscriber_count
        );
    }
    Ok(())
}
