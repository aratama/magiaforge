use crate::language::Dict;

// 一行は日本語で18文字まで

// UI
pub const CLICK_TO_START: Dict<&'static str> = Dict {
    ja: "クリックでスタート",
    en: "Click to Start",
    zh_cn: "点击开始",
    es: "Haz clic para empezar",
    fr: "Cliquez pour commencer",
};

pub const DISCOVERED_SPELLS: Dict<&'static str> = Dict {
    ja: "発見した呪文",
    en: "Discovered Spells",
    zh_cn: "发现的咒语",
    es: "Hechizos Descubiertos",
    fr: "Sorts Découverts",
};

pub const UNPEID: Dict<&'static str> = Dict {
    ja: "未精算",
    en: "Unpaid",
    zh_cn: "未付款",
    es: "No Pagado",
    fr: "Non Payé",
};

pub const INPUT_YOUR_NAME: Dict<&'static str> = Dict {
    ja: "名前を入力してください",
    en: "Input Your Name",
    zh_cn: "输入你的名字",
    es: "Introduce tu nombre",
    fr: "Entrez votre nom",
};

pub const START: Dict<&'static str> = Dict {
    ja: "スタート",
    en: "Start",
    zh_cn: "开始",
    es: "Comenzar",
    fr: "Commencer",
};

pub const PAUSED: Dict<&'static str> = Dict {
    ja: "ポーズ中",
    en: "Paused",
    zh_cn: "暂停",
    es: "Pausado",
    fr: "En Pause",
};

pub const FULLSCREEN: Dict<&'static str> = Dict {
    ja: "フルスクリーン",
    en: "Fullscreen",
    zh_cn: "全屏",
    es: "Pantalla Completa",
    fr: "Plein Écran",
};

pub const ON: Dict<&'static str> = Dict {
    ja: "オン",
    en: "On",
    zh_cn: "开",
    es: "Encendido",
    fr: "Activé",
};

pub const OFF: Dict<&'static str> = Dict {
    ja: "オフ",
    en: "Off",
    zh_cn: "关",
    es: "Apagado",
    fr: "Désactivé",
};

pub const BGM_VOLUE: Dict<&'static str> = Dict {
    ja: "BGM音量",
    en: "BGM Volume",
    zh_cn: "背景音乐音量",
    es: "Volumen de BGM",
    fr: "Volume BGM",
};

pub const SFX_VOLUME: Dict<&'static str> = Dict {
    ja: "効果音量",
    en: "SFX Volume",
    zh_cn: "音效音量",
    es: "Volumen de SFX",
    fr: "Volume SFX",
};

pub const RESUME: Dict<&'static str> = Dict {
    ja: "再開",
    en: "BACK",
    zh_cn: "恢复",
    es: "Reanudar",
    fr: "Reprendre",
};

pub const RETURN_TO_TITLE: Dict<&'static str> = Dict {
    ja: "タイトルに戻る",
    en: "Return to Title",
    zh_cn: "返回标题",
    es: "Volver al Título",
    fr: "Retour au Titre",
};

pub const SORT: Dict<&'static str> = Dict {
    ja: "並び替え",
    en: "Sort",
    zh_cn: "排序",
    es: "Ordenar",
    fr: "Trier",
};

// セリフ

pub const HELLO: Dict<&'static str> = Dict {
    ja: "おや、魔法使いか。\nここはぼくらの商人キャンプだよ。\n来客は歓迎さ。",
    en: "Oh, a witch.\nThis is our merchant camp.\nGuests are welcome.",
    zh_cn: "哦，一个女巫。\n这是我们的商人营地。\n欢迎客人。",
    es: "Oh, una bruja.\nEste es nuestro campamento de comerciantes.\nLos invitados son bienvenidos.",
    fr: "Oh, une sorcière.\nCeci est notre camp de marchands.\nLes invités sont les bienvenus.",
};

pub const HELLO_RABBITS: Dict<&'static str> = Dict {
    ja: "ほかの仲間たちにもぜひあいさつしていってくれ。",
    en: "Please say hello to the other rabbits.",
    zh_cn: "请向其他兔子问好。",
    es: "Por favor, saluda a los otros conejos.",
    fr: "Veuillez dire bonjour aux autres lapins.",
};

pub const SINGLEPLAY: Dict<&'static str> = Dict {
    ja: "その魔法陣は地下迷宮の入り口だよ。\n行くなら気をつけてね。\nあなたは魔法使いだから\n大丈夫だと思うけど。",
    en: "This is the entrance to the underground labyrinth.\nBe careful.\nI think you'll be fine\nbecause you are a witch.",
    zh_cn: "这是地下迷宫的入口。\n小心。\n我认为你会没事的\n因为你是女巫。",
    es: "Esta es la entrada al laberinto subterráneo.\nTen cuidado.\nCreo que estarás bien\nporque eres una bruja.",
    fr: "Ceci est l'entrée du labyrinthe souterrain.\nFaites attention.\nJe pense que vous serez bien\nparce que vous êtes une sorcière.",
};

pub const WITCHES_ARE: Dict<&'static str> = Dict {
    ja: "昔はこの島にも多くのヒト族がいたらしいが、今は魔法使いが時折訪れるくらいさ。君たち魔法使いはこの地底でいったい何を探しているんだい？",
    en: "There used to be many humans on this island but now only witches occasionally visit. What are you witches looking for in the depths?",
    zh_cn: "这个岛上曾经有很多人类，但现在只有女巫偶尔会来访。你们女巫在深处寻找什么？",
    es: "Solía haber muchos humanos en esta isla, pero ahora solo las brujas la visitan ocasionalmente. ¿Qué buscan ustedes, las brujas, en las profundidades?",
    fr: "Il y avait autrefois beaucoup d'humains sur cette île, mais maintenant seules les sorcières la visitent occasionnellement. Que cherchez-vous, sorcières, dans les profondeurs?",
};

pub const HUGE_SLIME: Dict<&'static str> = Dict {
    ja: "ところで最近、地下迷宮にとても大きなスライムが現れてね。大暴れして困っているんだ。",
    en: "Lately, a huge slime has appeared in the labyrinth. It's causing a lot of trouble.",
    zh_cn: "最近，一个巨大的史莱姆出现在迷宫里。它造成了很多麻烦。",
    es: "Últimamente, un gran slime ha aparecido en el laberinto. Está causando muchos problemas.",
    fr: "Dernièrement, un énorme slime est apparu dans le labyrinthe. Il cause beaucoup de problèmes.",
};

pub const HUGE_SLIME2: Dict<&'static str> = Dict {
    ja: "なにしろぼくらは地下迷宮で遺物を拾って生計を立てているからね。あんなのがうろついていたら落ち着いて探索もできやしない。",
    en: "After all, we make a living by picking up relics in the labyrinth. If such a thing is wandering around, we can't explore calmly.",
    zh_cn: "毕竟，我们是靠在迷宫里捡遗物谋生的。如果这样的东西四处游荡，我们就无法平静地探索。",
    es: "Después de todo, nos ganamos la vida recogiendo reliquias en el laberinto. Si algo así está deambulando, no podemos explorar con calma.",
    fr: "Après tout, nous gagnons notre vie en ramassant des reliques dans le labyrinthe. Si une telle chose erre, nous ne pouvons pas explorer calmement.",
};

pub const HUGE_SLIME3: Dict<&'static str> = Dict {
    ja: "あなたがあのスライムを討伐してくれたら、とても助かるんだけど。",
    en: "If you could defeat that slime, I would be very grateful.",
    zh_cn: "如果你能打败那个史莱姆，我会非常感激。",
    es: "Si pudieras derrotar a ese slime, estaría muy agradecido.",
    fr: "Si vous pouviez vaincre ce slime, je vous en serais très reconnaissant.",
};

pub const HUGE_SLIME4: Dict<&'static str> = Dict {
    ja: "その大きなスライムは体当たりで攻撃してくるが、足が早ければ逃げるのは難しくない。",
    en: "The huge slime attacks with a body blow, but if you have fast legs, it's not hard to escape.",
    zh_cn: "巨大的史莱姆会用身体冲击攻击，但如果你的腿很快",
    es: "El gran slime ataca con un golpe de cuerpo, pero si tienes piernas rápidas, no es difícil escapar.",
    fr: "Le gros slime attaque avec un coup de corps, mais si vous avez des jambes rapides, il n'est pas difficile de s'échapper.",
};

pub const HUGE_SLIME5: Dict<&'static str> = Dict {
    ja: "それと、あいつは仲間の小さなスライムを呼び寄せるんだ。囲まれると逃げ道を失う。周囲のスライムは素早く倒したほうがいい。",
    en: "And it calls small slimes to its side. If you are surrounded, you will lose your escape route. It's better to defeat the surrounding slimes quickly.",
    zh_cn: "它会召唤小史莱姆来帮忙。如果你被包围，你就会失去逃跑的路线。最好快速击败周围的史莱姆。",
    es: "Y llama a pequeños slimes a su lado. Si estás rodeado, perderás tu ruta de escape. Es mejor derrotar rápidamente a los slimes circundantes.",
    fr: "Et il appelle des petits slimes à ses côtés. Si vous êtes entouré, vous perdrez votre route d'évasion. Il vaut mieux vaincre rapidement les slimes environnants.",
};

pub const MULTIPLAY: Dict<&'static str> = Dict {
    ja: "この先はマルチプレイ用レベルだよ。\n気軽に遊んでいってね。\n誰かいるかはわからないけど。",
    en: "It seems that this is a level for multiplayer.\nFeel free to play.\nI don't know if anyone is there.",
    zh_cn: "这似乎是一个多人游戏的级别。\n随意玩。\n我不知道有没有人在那里。",
    es: "Parece que este es un nivel para multijugador.\nSiéntete libre de jugar.\nNo sé si hay alguien allí.",
    fr: "Il semble que ce soit un niveau pour multijoueur.\nN'hésitez pas à jouer.\nJe ne sais pas si quelqu'un est là.",
};

pub const TRAINING_RABBIT: Dict<&'static str> = Dict {
    ja: "キミも強くなりたいのかい？\nここで練習していくといい。\nサンドバッグくんたちが相手になってくれる。",
    en: "Do you want to be strong?\nIt's good to practice here.\nThe sandbag guys will be your opponent.",
    zh_cn: "你想变强吗？\n在这里练习是很好的。\n沙袋们会成为你的对手。",
    es: "¿Quieres ser fuerte?\nEs bueno practicar aquí.\nLos sacos de arena serán tus oponentes.",
    fr: "Voulez-vous devenir fort?\nC'est bien de s'entraîner ici.\nLes sacs de sable seront vos adversaires.",
};

pub const SPELL_LIST1: Dict<&'static str> = Dict {
    ja: "私は魔法使いたちの操る呪文に興味があってね。君の知っている呪文について教えてくれないか？",
    en: "I'm interested in the spells cast by witches. Can you tell me about the spells you know?",
    zh_cn: "我对女巫施展的咒语很感兴趣。你能告诉我你所知道的咒语吗？",
    es: "Estoy interesado en los hechizos lanzados por las brujas. ¿Puedes contarme sobre los hechizos que conoces?",
    fr: "Je suis intéressé par les sorts lancés par les sorcières. Pouvez-vous me parler des sorts que vous connaissez?",
};

pub const SPELL_LIST2: Dict<&'static str> = Dict {
    ja: "……ふうむ、実に興味深い。もし新たな魔法を見つけたらぜひ教えてくれ。",
    en: "Hmmmm, very interesting. Please let me know if you find a new spell.",
    zh_cn: "嗯嗯，非常有趣。如果您发现新咒语，请告诉我。",
    es: "Hmmmm, muy interesante. Por favor, avísame si encuentras un nuevo hechizo.",
    fr: "Hmmmm, très intéressant. Veuillez me faire savoir si vous trouvez un nouveau sort.",
};

pub const SPELL_LIST3: Dict<&'static str> = Dict {
    ja: "皆にもその呪文を集めるよう伝えておこう。次からその呪文が店にも並ぶようになるはずだ。",
    en: "I'll tell everyone to collect that spells. It should be available in the shop from now on.",
    zh_cn: "我会告诉大家收集咒语。从现在开始，它应该可以在商店里买到。",
    es: "Le diré a todos que recojan esos hechizos. Debería estar disponible en la tienda a partir de ahora.",
    fr: "Je dirai à tout le monde de collecter ces sorts. Ils devraient être disponibles dans la boutique à partir de maintenant.",
};

pub const SHOP_RABBIT: Dict<&'static str> = Dict {
    ja: "いらっしゃいませ！\n欲しい商品があったら\n持ってきてくださいね",
    en: "Welcome!\nBring me the item\nyou want to buy.",
    zh_cn: "欢迎！\n给我带来\n你想买的商品。",
    es: "¡Bienvenido!\nTráeme el artículo\nque quieres comprar.",
    fr: "Bienvenue!\nApportez-moi l'article\nque vous voulez acheter.",
};

pub fn shop_rabbit(golds: u32) -> Dict<String> {
    Dict {
        ja: format!(
            "合計{}ゴールドのお買い上げです！\nありがとうございます",
            golds
        ),
        en: format!("Your total is {} Golds\nThank you", golds),
        zh_cn: format!("您的总点数为 {} 枚金牌。谢谢你！", golds),
        es: format!("Tu total es de {} oros\nGracias", golds),
        fr: format!("Votre total est de {} pièces d'or\nMerci", golds),
    }
}

pub fn too_few_golds(golds: u32) -> Dict<String> {
    Dict {
        ja: format!(
            "おいおい\n{}ゴールド足りませんよ\n買わない商品は\n戻しておいてくださいね",
            golds
        ),
        en: format!(
            "Hey, hey!\nYou are {} Golds short!\nPut it back that you won't buy",
            golds
        ),
        zh_cn: format!("嘿，嘿！\n你少了{}枚金牌！\n把你不买的东西放回去", golds),
        es: format!(
            "¡Oye, oye!\nTe faltan {} oros!\nDevuelve lo que no vas a comprar",
            golds
        ),
        fr: format!(
            "Hé, hé!\nIl vous manque {} pièces d'or!\nRemettez ce que vous n'achetez pas",
            golds
        ),
    }
}

pub const PAY_FIRST: Dict<&'static str> = Dict {
    ja: "おいおい、代金を払ってから行ってくれ",
    en: "Hey Hey, pay first before you go",
    zh_cn: "嘿，嘿，先付钱再走",
    es: "Oye, oye, paga antes de irte",
    fr: "Hé hé, payez d'abord avant de partir",
};

// 地名 /////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub const LEVEL0: Dict<&'static str> = Dict {
    ja: "見捨てられた工房",
    en: "Abandoned Workshop",
    zh_cn: "废弃的车间",
    es: "Taller Abandonado",
    fr: "Atelier Abandonné",
};

pub const LEVEL1: Dict<&'static str> = Dict {
    ja: "図書館跡",
    en: "Library Ruins",
    zh_cn: "图书馆废墟",
    es: "Ruinas de la Biblioteca",
    fr: "Ruines de la Bibliothèque",
};

pub const LEVEL2: Dict<&'static str> = Dict {
    ja: "地下草原",
    en: "Underground Grassland",
    zh_cn: "地下草原",
    es: "Pradera Subterránea",
    fr: "Prairie Souterraine",
};

pub const LEVEL3: Dict<&'static str> = Dict {
    ja: "古城",
    en: "Ancient Castle",
    zh_cn: "古城",
    es: "Castillo Antiguo",
    fr: "Château Ancien",
};

pub const LEVEL4: Dict<&'static str> = Dict {
    ja: "スライムの巣窟",
    en: "Slime Nest",
    zh_cn: "史莱姆巢穴",
    es: "Nido de Slimes",
    fr: "Nid de Slimes",
};

pub const MULTIPLAY_ARENA: Dict<&'static str> = Dict {
    ja: "対決の洞窟",
    en: "Arena Cave",
    zh_cn: "竞技场洞穴",
    es: "Cueva de la Arena",
    fr: "Grotte de l'Arène",
};

pub const UNKNOWN_LEVEL: Dict<&'static str> = Dict {
    ja: "不明",
    en: "Unknown",
    zh_cn: "未知",
    es: "Desconocido",
    fr: "Inconnu",
};
