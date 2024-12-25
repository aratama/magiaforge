use crate::language::Dict;

// 一行は日本語で18文字まで

pub const HELLO: Dict<&'static str> = Dict {
    ja: "やあ、魔法使いさん。\nここはぼくらの商人キャンプだよ。\n来客は歓迎さ。",
    en: "Hi, you look unfamiliar.\nThis is our merchant camp.\nGuests are welcome.",
};

pub const HELLO_RABBITS: Dict<&'static str> = Dict {
    ja: "ほかの仲間たちにもぜひあいさつしていってくれ。",
    en: "Please say hello to the other rabbits.",
};

pub const SINGLEPLAY: Dict<&'static str> = Dict {
    ja: "その魔法陣は地下迷宮の入り口だよ。\n行くなら気をつけてね。\nあなたは魔法使いだから\n大丈夫だと思うけど。",
    en: "This is the entrance to the underground labyrinth.\nBe careful.\nI think you'll be fine\nbecause you are a witch.",
};

pub const WITCHES_ARE: Dict<&'static str> = Dict {
    ja: "昔はこの島にも多くのヒト族がいたらしいが、今は魔法使いが時折訪れるくらいさ。君たち魔法使いはこの地底でいったい何を探しているんだい？",
    en: "There used to be many humans on this island but now only witches occasionally visit. What are you witches looking for in the depths?",
};

pub const HUGE_SLIME: Dict<&'static str> = Dict {
    ja: "ところで最近、地下迷宮にとても大きなスライムが現れてね。大暴れして困っているんだ。",
    en: "Lately, a huge slime has appeared in the labyrinth. It's causing a lot of trouble.",
};

pub const HUGE_SLIME2: Dict<&'static str> = Dict {
    ja: "なにしろぼくらは地下迷宮で遺物を拾って生計を立てているからね。あんなのがうろついていたら落ち着いて探索もできやしない。",
    en: "After all, we make a living by picking up relics in the labyrinth. If such a thing is wandering around, we can't explore calmly.",
};

pub const HUGE_SLIME3: Dict<&'static str> = Dict {
    ja: "あなたがあのスライムを討伐してくれたら、とても助かるんだけど。",
    en: "If you could defeat that slime, I would be very grateful.",
};

pub const HUGE_SLIME4: Dict<&'static str> = Dict {
    ja: "その大きなスライムは体当たりで攻撃してくるが、足が早ければ逃げるのは難しくない。",
    en: "The huge slime attacks with a body blow, but if you have fast legs, it's not hard to escape.",
};

pub const HUGE_SLIME5: Dict<&'static str> = Dict {
    ja: "それと、あいつは仲間の小さなスライムを呼び寄せるんだ。囲まれると逃げ道を失う。周囲のスライムは素早く倒したほうがいい。",
    en: "And it calls small slimes to its side. If you are surrounded, you will lose your escape route. It's better to defeat the surrounding slimes quickly.",
};

pub const MULTIPLAY : Dict<&'static str> = Dict {
    ja: "この先はマルチプレイ用レベルだよ。\n気軽に遊んでいってね。\n誰かいるかはわからないけど。",
    en: "It seems that this is a level for multiplayer.\nFeel free to play.\nI don't know if anyone is there.",
};

pub const TRAINING_RABBIT : Dict<&'static str> =  Dict {
    ja: "キミも強くなりたいのかい？\nここで練習していくといい。\nサンドバッグくんたちが相手になってくれる。",
    en: "Do you want to be strong?\nIt's good to practice here.\nThe sandbag guys will be your opponent.",
};

pub const SHOP_RABBIT: Dict<&'static str> = Dict {
    ja: "いらっしゃいませ！\n欲しい商品があったら\n持ってきてくださいね",
    en: "Welcome!\nBring me the item\nyou want to buy",
};

pub fn shop_rabbit(golds: u32) -> Dict<String> {
    Dict {
        ja: format!(
            "合計{}ゴールドのお買い上げです！\nありがとうございます",
            golds
        ),
        en: format!("Your total is {} Golds\nThank you", golds),
    }
}

pub fn too_few_golds(golds: u32) -> Dict<String> {
    Dict {
        ja: format!(
            "おいおい\n{}ゴールド足りませんよ\n買わない商品は\n戻しておいてくださいね",
            golds
        ),
        en: format!(
            "Hey, hey!\nYou are {} Golds short!\nPut it back that you woun't buy",
            golds
        ),
    }
}

// 地名 /////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub const LEVEL0: Dict<&'static str> = Dict {
    ja: "見捨てられた工房",
    en: "Abandoned Workshop",
};

pub const LEVEL1: Dict<&'static str> = Dict {
    ja: "図書館跡",
    en: "Library Ruins",
};

pub const LEVEL2: Dict<&'static str> = Dict {
    ja: "洞窟",
    en: "Cave",
};

pub const LEVEL3: Dict<&'static str> = Dict {
    ja: "スライムの巣窟",
    en: "Slime Nest",
};

pub const MULTIPLAY_ARENA: Dict<&'static str> = Dict {
    ja: "対決の洞窟",
    en: "Arena Cave",
};

pub const UNKNOWN_LEVEL: Dict<&'static str> = Dict {
    ja: "不明",
    en: "Unknown",
};
