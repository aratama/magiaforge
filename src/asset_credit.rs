use bevy::audio::{AudioSink, AudioSource};
use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct AssetCredit {
    pub authoer: &'static str,
    pub title: &'static str,
    pub appendix: &'static str,
}

pub fn asset_to_credit(handle: &Handle<AudioSource>) -> AssetCredit {
    if let Some(path) = handle.path() {
        // info!("label {:?}", path.label());
        info!("label_cow {:?}", path.label_cow());
        info!("path {:?}", path.path());
        info!("source {:?}", path.source());
        info!("get_full_extension {:?}", path.get_full_extension());

        match path
            .path()
            .file_stem()
            .and_then(|s| s.to_str())
            .unwrap_or("")
        {
            // 拠点
            "茫漠たる庭" => AssetCredit {
                authoer: "のる",
                title: "茫漠たる庭",
                appendix: "",
            },
            // タイトル画面ではポーズメニューを開けないため、
            // 洞窟環境音のときにタイトル画面BGMのクレジットを表示
            "水のしたたる洞窟" => AssetCredit {
                authoer: "のる",
                title: "茫漠たる庭",
                appendix: "(タイトル画面BGM)",
            },

            "最果てのルージュ" => AssetCredit {
                authoer: "のる",
                title: "最果てのルージュ",
                appendix: "",
            },

            // 通常
            "荒れ地の先へ" => AssetCredit {
                authoer: "松浦洋介",
                title: "荒れ地の先へ",
                appendix: "",
            },

            "Tides_of_Adventure" => AssetCredit {
                authoer: "K’z Art Storage",
                title: "Tides of Adventure",
                appendix: "",
            },

            "ダンジョンを踏破せし者" => AssetCredit {
                authoer: "こーち",
                title: "ダンジョンを踏破せし者",
                appendix: "",
            },

            "森のいざない" => AssetCredit {
                authoer: "MATSU",
                title: "森のいざない",
                appendix: "",
            },

            "迷宮" => AssetCredit {
                authoer: "オオヒラセイジ",
                title: "迷宮",
                appendix: "",
            },

            "忘れられた神殿" => AssetCredit {
                authoer: "KK",
                title: "忘れられた神殿",
                appendix: "",
            },

            "midnight-forest-184304" => AssetCredit {
                authoer: "Syouki Takahashi",
                title: "Midnight Forest",
                appendix: "",
            },

            // ボス
            "悪魔との戦闘" => AssetCredit {
                authoer: "今川彰人オーケストラ",
                title: "悪魔との戦闘",
                appendix: "",
            },

            "アクション・バトル" => AssetCredit {
                authoer: "田中芳典",
                title: "アクション・バトル",
                appendix: "",
            },

            "Decisive_Battle" => AssetCredit {
                authoer: "Make a field Music",
                title: "Decisive Battle",
                appendix: "",
            },

            "炎神の吐息" => AssetCredit {
                authoer: "こおろぎ",
                title: "炎神の吐息",
                appendix: "",
            },

            "Sacred_Sacrifice" => AssetCredit {
                authoer: "ilodolly",
                title: "Sacred Sacrifice",
                appendix: "",
            },

            "final-battle-trailer-music-217488" => AssetCredit {
                authoer: "Ihor_Koliako",
                title: "FINAL BATTLE",
                appendix: "",
            },

            "human-vs-machine-dark-orchestral-cinematic-epic-action-271968" => AssetCredit {
                authoer: "Kulakovka",
                title: "Human vs Machine",
                appendix: "",
            },

            _ => {
                warn!("no credit for the audio: {:?}", path.path());
                AssetCredit {
                    authoer: "",
                    title: "",
                    appendix: "",
                }
            }
        }
    } else {
        warn!("no path in audio handle");
        AssetCredit {
            authoer: "",
            title: "",
            appendix: "",
        }
    }
}
