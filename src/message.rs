use crate::language::Dict;

// 一行は日本語で18文字まで

// UI
pub const CLICK_TO_START: Dict<&'static str> = Dict {
    ja: "クリックでスタート",
    en: "Click to Start",
    zh_cn: "点击开始",
    es: "Haz clic para empezar",
    fr: "Cliquez pour commencer",
    pt: "Clique para começar",
    de: "Klicken zum Starten",
    ko: "클릭하여 시작",
    ru: "Нажмите, чтобы начать",
};

pub const DISCOVERED_SPELLS: Dict<&'static str> = Dict {
    ja: "発見した呪文",
    en: "Discovered Spells",
    zh_cn: "发现的咒语",
    es: "Hechizos Descubiertos",
    fr: "Sorts Découverts",
    pt: "Feitiços Descobertos",
    de: "Entdeckte Zauber",
    ko: "발견된 주문",
    ru: "Обнаруженные заклинания",
};

pub const UNPAID: Dict<&'static str> = Dict {
    ja: "未精算",
    en: "Unpaid",
    zh_cn: "未付款",
    es: "No Pagado",
    fr: "Non Payé",
    pt: "Não Pago",
    de: "Unbezahlt",
    ko: "미지불",
    ru: "Неоплаченный",
};

pub const INPUT_YOUR_NAME: Dict<&'static str> = Dict {
    ja: "名前を入力してください",
    en: "Input Your Name",
    zh_cn: "输入你的名字",
    es: "Introduce tu nombre",
    fr: "Entrez votre nom",
    pt: "Digite seu nome",
    de: "Geben Sie Ihren Namen ein",
    ko: "이름을 입력하세요",
    ru: "Введите ваше имя",
};

pub const START: Dict<&'static str> = Dict {
    ja: "スタート",
    en: "Start",
    zh_cn: "开始",
    es: "Comenzar",
    fr: "Commencer",
    pt: "Começar",
    de: "Starten",
    ko: "시작",
    ru: "Начать",
};

pub const PAUSED: Dict<&'static str> = Dict {
    ja: "ポーズ中",
    en: "Paused",
    zh_cn: "暂停",
    es: "Pausado",
    fr: "En Pause",
    pt: "Pausado",
    de: "Pausiert",
    ko: "일시 중지",
    ru: "Пауза",
};

pub const FULLSCREEN: Dict<&'static str> = Dict {
    ja: "フルスクリーン",
    en: "Fullscreen",
    zh_cn: "全屏",
    es: "Pantalla Completa",
    fr: "Plein Écran",
    pt: "Tela Cheia",
    de: "Vollbild",
    ko: "전체 화면",
    ru: "Полноэкранный режим",
};

pub const ON: Dict<&'static str> = Dict {
    ja: "オン",
    en: "On",
    zh_cn: "开",
    es: "Encendido",
    fr: "Activé",
    pt: "Ligado",
    de: "Ein",
    ko: "켜기",
    ru: "Вкл",
};

pub const OFF: Dict<&'static str> = Dict {
    ja: "オフ",
    en: "Off",
    zh_cn: "关",
    es: "Apagado",
    fr: "Désactivé",
    pt: "Desligado",
    de: "Aus",
    ko: "끄기",
    ru: "Выкл",
};

pub const BGM_VOLUME: Dict<&'static str> = Dict {
    ja: "BGM音量",
    en: "BGM Volume",
    zh_cn: "背景音乐音量",
    es: "Volumen de BGM",
    fr: "Volume BGM",
    pt: "Volume do BGM",
    de: "BGM-Lautstärke",
    ko: "배경 음악 볼륨",
    ru: "Громкость BGM",
};

pub const SFX_VOLUME: Dict<&'static str> = Dict {
    ja: "効果音量",
    en: "SFX Volume",
    zh_cn: "音效音量",
    es: "Volumen de SFX",
    fr: "Volume SFX",
    pt: "Volume de SFX",
    de: "SFX-Lautstärke",
    ko: "효과음 볼륨",
    ru: "Громкость SFX",
};

pub const RESUME: Dict<&'static str> = Dict {
    ja: "再開",
    en: "Resume",
    zh_cn: "恢复",
    es: "Reanudar",
    fr: "Reprendre",
    pt: "Retomar",
    de: "Fortsetzen",
    ko: "다시 시작",
    ru: "Продолжить",
};

pub const RETURN_TO_TITLE: Dict<&'static str> = Dict {
    ja: "タイトルに戻る",
    en: "Return to Title",
    zh_cn: "返回标题",
    es: "Volver al Título",
    fr: "Retour au Titre",
    pt: "Voltar ao Título",
    de: "Zurück zum Titel",
    ko: "타이틀로 돌아가기",
    ru: "Вернуться к заголовку",
};

pub const SORT: Dict<&'static str> = Dict {
    ja: "並び替え",
    en: "Sort",
    zh_cn: "排序",
    es: "Ordenar",
    fr: "Trier",
    pt: "Ordenar",
    de: "Sortieren",
    ko: "정렬",
    ru: "Сортировать",
};

pub const HELLO: Dict<&'static str> = Dict {
    ja: "おや、魔法使いか。\nここはぼくらの商人キャンプだよ。\n来客は歓迎さ。",
    en: "Oh, a witch.\nThis is our merchant camp.\nGuests are welcome.",
    zh_cn: "哦，一个女巫。\n这是我们的商人营地。\n欢迎客人。",
    es: "Oh, una bruja.\nEste es nuestro campamento de comerciantes.\nLos invitados son bienvenidos.",
    fr: "Oh, une sorcière.\nCeci est notre camp de marchands.\nLes invités sont les bienvenus.",
    pt: "Oh, uma bruxa.\nEste é nosso acampamento de mercadores.\nOs convidados são bem-vindos.",
    de: "Oh, eine Hexe.\nDies ist unser Händlerlager.\nGäste sind willkommen.",
    ko: "오, 마녀야.\n여기는 우리 상인 캠프야.\n손님을 환영해.",
    ru: "О, ведьма.\nЭто наш лагерь торговцев.\nГостям рады.",
};

pub const HELLO_RABBITS: Dict<&'static str> = Dict {
    ja: "ほかの仲間たちにもぜひあいさつしていってくれ。",
    en: "Please say hello to the other rabbits.",
    zh_cn: "请向其他兔子问好。",
    es: "Por favor, saluda a los otros conejos.",
    fr: "Veuillez dire bonjour aux autres lapins.",
    pt: "Por favor, cumprimente os outros coelhos.",
    de: "Bitte grüße die anderen Kaninchen.",
    ko: "다른 토끼들에게도 인사해 주세요.",
    ru: "Пожалуйста, поздоровайтесь с другими кроликами.",
};

pub const SINGLEPLAY: Dict<&'static str> = Dict {
    ja: "その魔法陣は地下迷宮の入り口だよ。\n行くなら気をつけてね。\nあなたは魔法使いだから\n大丈夫だと思うけど。",
    en: "This is the entrance to the underground labyrinth.\nBe careful.\nI think you'll be fine\nbecause you are a witch.",
    zh_cn: "这是地下迷宫的入口。\n小心。\n我认为你会没事的\n因为你是女巫。",
    es: "Esta es la entrada al laberinto subterráneo.\nTen cuidado.\nCreo que estarás bien\nporque eres una bruja.",
    fr: "Ceci est l'entrée du labyrinthe souterrain.\nFaites attention.\nJe pense que vous serez bien\nparce que vous êtes une sorcière.",
    pt: "Esta é a entrada para o labirinto subterrâneo.\nTenha cuidado.\nAcho que você ficará bem\nporque você é uma bruxa.",
    de: "Dies ist der Eingang zum unterirdischen Labyrinth.\nSei vorsichtig.\nIch denke, du wirst in Ordnung sein,\nweil du eine Hexe bist.",
    ko: "여기는 지하 미로의 입구야.\n조심해.\n너는 마녀니까 괜찮을 거야.",
    ru: "Это вход в подземный лабиринт.\nБудь осторожен.\nДумаю, с тобой все будет в порядке,\nпотому что ты ведьма.",
};

pub const WITCHES_ARE: Dict<&'static str> = Dict {
    ja: "昔はこの島にも多くのヒト族がいたらしいが、今は魔法使いが時折訪れるくらいさ。君たち魔法使いはこの地底でいったい何を探しているんだい？",
    en: "There used to be many humans on this island but now only witches occasionally visit. What are you witches looking for in the depths?",
    zh_cn: "这个岛上曾经有很多人类，但现在只有女巫偶尔会来访。你们女巫在深处寻找什么？",
    es: "Solía haber muchos humanos en esta isla, pero ahora solo las brujas la visitan ocasionalmente. ¿Qué buscan ustedes, las brujas, en las profundidades?",
    fr: "Il y avait autrefois beaucoup d'humains sur cette île, mais maintenant seules les sorcières la visitent occasionnellement. Que cherchez-vous, sorcières, dans les profondeurs?",
    pt: "Costumava haver muitos humanos nesta ilha, mas agora apenas bruxas a visitam ocasionalmente. O que vocês, bruxas, estão procurando nas profundezas?",
    de: "Früher gab es viele Menschen auf dieser Insel, aber jetzt besuchen nur noch gelegentlich Hexen. Was sucht ihr Hexen in den Tiefen?",
    ko: "예전에는 이 섬에 많은 인간이 있었지만 지금은 가끔 마녀들만 방문해. 너희 마녀들은 깊은 곳에서 무엇을 찾고 있니?",
    ru: "Раньше на этом острове было много людей, но теперь его изредка посещают только ведьмы. Что вы, ведьмы, ищете в глубинах?",
};

pub const HUGE_SLIME: Dict<&'static str> = Dict {
    ja: "ところで最近、地下迷宮にとても大きなスライムが現れてね。大暴れして困っているんだ。",
    en: "Lately, a huge slime has appeared in the labyrinth. It's causing a lot of trouble.",
    zh_cn: "最近，一个巨大的史莱姆出现在迷宫里。它造成了很多麻烦。",
    es: "Últimamente, un gran slime ha aparecido en el laberinto. Está causando muchos problemas.",
    fr: "Dernièrement, un énorme slime est apparu dans le labyrinthe. Il cause beaucoup de problèmes.",
    pt: "Ultimamente, um grande slime apareceu no labirinto. Está causando muitos problemas.",
    de: "In letzter Zeit ist ein riesiger Schleim im Labyrinth aufgetaucht. Es verursacht viele Probleme.",
    ko: "최근에 미로에 거대한 슬라임이 나타났어. 많은 문제를 일으키고 있어.",
    ru: "В последнее время в лабиринте появился огромный слизень. Он вызывает много проблем.",
};

pub const HUGE_SLIME2: Dict<&'static str> = Dict {
    ja: "なにしろぼくらは地下迷宮で遺物を拾って生計を立てているからね。あんなのがうろついていたら落ち着いて探索もできやしない。",
    en: "After all, we make a living by picking up relics in the labyrinth. If such a thing is wandering around, we can't explore calmly.",
    zh_cn: "毕竟，我们是靠在迷宫里捡遗物谋生的。如果这样的东西四处游荡，我们就无法平静地探索。",
    es: "Después de todo, nos ganamos la vida recogiendo reliquias en el laberinto. Si algo así está deambulando, no podemos explorar con calma.",
    fr: "Après tout, nous gagnons notre vie en ramassant des reliques dans le labyrinthe. Si une telle chose erre, nous ne pouvons pas explorer calmement.",
    pt: "Afinal, ganhamos a vida recolhendo relíquias no labirinto. Se algo assim estiver vagando, não podemos explorar calmamente.",
    de: "Schließlich verdienen wir unseren Lebensunterhalt, indem wir Relikte im Labyrinth aufsammeln. Wenn so etwas herumwandert, können wir nicht ruhig erkunden.",
    ko: "결국 우리는 미로에서 유물을 주워 생계를 유지해. 그런 것이 돌아다니면 우리는 차분하게 탐험할 수 없어.",
    ru: "В конце концов, мы зарабатываем на жизнь, собирая реликвии в лабиринте. Если такое существо будет бродить вокруг, мы не сможем спокойно исследовать.",
};

pub const HUGE_SLIME3: Dict<&'static str> = Dict {
    ja: "あなたがあのスライムを討伐してくれたら、とても助かるんだけど。",
    en: "If you could defeat that slime, I would be very grateful.",
    zh_cn: "如果你能打败那个史莱姆，我会非常感激。",
    es: "Si pudieras derrotar a ese slime, estaría muy agradecido.",
    fr: "Si vous pouviez vaincre ce slime, je vous en serais très reconnaissant.",
    pt: "Se você pudesse derrotar aquele slime, eu ficaria muito grato.",
    de: "Wenn du diesen Schleim besiegen könntest, wäre ich dir sehr dankbar.",
    ko: "네가 그 슬라임을 물리쳐 준다면 정말 고마울 거야.",
    ru: "Если бы ты мог победить этого слизня, я был бы очень благодарен.",
};

pub const HUGE_SLIME4: Dict<&'static str> = Dict {
    ja: "その大きなスライムは体当たりで攻撃してくるが、足が早ければ逃げるのは難しくない。",
    en: "The huge slime attacks with a body blow, but if you have fast legs, it's not hard to escape.",
    zh_cn: "巨大的史莱姆会用身体冲击攻击，但如果你的腿很快",
    es: "El gran slime ataca con un golpe de cuerpo, pero si tienes piernas rápidas, no es difícil escapar.",
    fr: "Le gros slime attaque avec un coup de corps, mais si vous avez des jambes rapides, il n'est pas difficile de s'échapper.",
    pt: "O grande slime ataca com um golpe corporal, mas se você tiver pernas rápidas, não é difícil escapar.",
    de: "Der riesige Schleim greift mit einem Körperstoß an, aber wenn du schnelle Beine hast, ist es nicht schwer zu entkommen.",
    ko: "거대한 슬라임은 몸통 박치기로 공격하지만, 다리가 빠르면 도망치는 것은 어렵지 않아.",
    ru: "Огромный слизень атакует телесным ударом, но если у тебя быстрые ноги, убежать несложно.",
};

pub const HUGE_SLIME5: Dict<&'static str> = Dict {
    ja: "それと、あいつは仲間の小さなスライムを呼び寄せるんだ。囲まれると逃げ道を失う。周囲のスライムは素早く倒したほうがいい。",
    en: "And it calls small slimes to its side. If you are surrounded, you will lose your escape route. It's better to defeat the surrounding slimes quickly.",
    zh_cn: "它会召唤小史莱姆来帮忙。如果你被包围，你就会失去逃跑的路线。最好快速击败周围的史莱姆。",
    es: "Y llama a pequeños slimes a su lado. Si estás rodeado, perderás tu ruta de escape. Es mejor derrotar rápidamente a los slimes circundantes.",
    fr: "Et il appelle des petits slimes à ses côtés. Si vous êtes entouré, vous perdrez votre route d'évasion. Il vaut mieux vaincre rapidement les slimes environnants.",
    pt: "E ele chama pequenos slimes para o seu lado. Se você estiver cercado, perderá sua rota de fuga. É melhor derrotar rapidamente os slimes ao redor.",
    de: "Und es ruft kleine Schleime zu sich. Wenn du umzingelt bist, verlierst du deinen Fluchtweg. Es ist besser, die umliegenden Schleime schnell zu besiegen.",
    ko: "그리고 작은 슬라임들을 불러들여. 둘러싸이면 도망갈 길을 잃게 돼. 주변의 슬라임들을 빨리 물리치는 것이 좋아.",
    ru: "И он зовет к себе маленьких слизней. Если ты окажешься окружен, ты потеряешь путь к бегству. Лучше быстро победить окружающих слизней.",
};

pub const MULTIPLAY: Dict<&'static str> = Dict {
    ja: "この先はマルチプレイ用レベルだよ。\n気軽に遊んでいってね。\n誰かいるかはわからないけど。",
    en: "It seems that this is a level for multiplayer.\nFeel free to play.\nI don't know if anyone is there.",
    zh_cn: "这似乎是一个多人游戏的级别。\n随意玩。\n我不知道有没有人在那里。",
    es: "Parece que este es un nivel para multijugador.\nSiéntete libre de jugar.\nNo sé si hay alguien allí.",
    fr: "Il semble que ce soit un niveau pour multijoueur.\nN'hésitez pas à jouer.\nJe ne sais pas si quelqu'un est là.",
    pt: "Parece que este é um nível para multijogador.\nSinta-se à vontade para jogar.\nNão sei se há alguém lá.",
    de: "Es scheint, dass dies ein Level für Mehrspieler ist.\nFühlen Sie sich frei zu spielen.\nIch weiß nicht, ob jemand da ist.",
    ko: "이것은 멀티플레이어를 위한 레벨인 것 같아.\n편하게 플레이해.\n누가 있는지 모르겠어.",
    ru: "Кажется, это уровень для многопользовательской игры.\nНе стесняйтесь играть.\nНе знаю, есть ли там кто-нибудь.",
};

pub const TRAINING_RABBIT: Dict<&'static str> = Dict {
    ja: "キミも強くなりたいのかい？\nここで練習していくといい。\nサンドバッグくんたちが相手になってくれる。",
    en: "Do you want to be strong?\nIt's good to practice here.\nThe sandbag guys will be your opponent.",
    zh_cn: "你想变强吗？\n在这里练习是很好的。\n沙袋们会成为你的对手。",
    es: "¿Quieres ser fuerte?\nEs bueno practicar aquí.\nLos sacos de arena serán tus oponentes.",
    fr: "Voulez-vous devenir fort?\nC'est bien de s'entraîner ici.\nLes sacs de sable seront vos adversaires.",
    pt: "Você quer ser forte?\nÉ bom praticar aqui.\nOs sacos de areia serão seus oponentes.",
    de: "Willst du stark werden?\nEs ist gut, hier zu üben.\nDie Sandsäcke werden deine Gegner sein.",
    ko: "강해지고 싶어?\n여기서 연습하는 것이 좋아.\n샌드백들이 상대가 되어줄 거야.",
    ru: "Хочешь стать сильным?\nЗдесь хорошо тренироваться.\nМешки с песком будут твоими противниками.",
};

pub const SPELL_LIST1: Dict<&'static str> = Dict {
    ja: "私は魔法使いたちの操る呪文に興味があってね。君の知っている呪文について教えてくれないか？",
    en: "I'm interested in the spells cast by witches. Can you tell me about the spells you know?",
    zh_cn: "我对女巫施展的咒语很感兴趣。你能告诉我你所知道的咒语吗？",
    es: "Estoy interesado en los hechizos lanzados por las brujas. ¿Puedes contarme sobre los hechizos que conoces?",
    fr: "Je suis intéressé par les sorts lancés par les sorcières. Pouvez-vous me parler des sorts que vous connaissez?",
    pt: "Estou interessado nos feitiços lançados pelas bruxas. Você pode me contar sobre os feitiços que conhece?",
    de: "Ich interessiere mich für die Zauber, die Hexen wirken. Kannst du mir von den Zaubern erzählen, die du kennst?",
    ko: "나는 마녀들이 사용하는 주문에 관심이 있어. 네가 알고 있는 주문에 대해 말해줄 수 있니?",
    ru: "Меня интересуют заклинания, которые используют ведьмы. Можешь рассказать мне о заклинаниях, которые ты знаешь?",
};

pub const SPELL_LIST2: Dict<&'static str> = Dict {
    ja: "……ふうむ、実に興味深い。もし新たな魔法を見つけたらぜひ教えてくれ。",
    en: "Hmmmm, very interesting. Please let me know if you find a new spell.",
    zh_cn: "嗯嗯，非常有趣。如果您发现新咒语，请告诉我。",
    es: "Hmmmm, muy interesante. Por favor, avísame si encuentras un nuevo hechizo.",
    fr: "Hmmmm, très intéressant. Veuillez me faire savoir si vous trouvez un nouveau sort.",
    pt: "Hmmmm, muito interessante. Por favor, me avise se encontrar um novo feitiço.",
    de: "Hmmmm, sehr interessant. Bitte lass mich wissen, wenn du einen neuen Zauber findest.",
    ko: "음, 매우 흥미로워. 새로운 주문을 발견하면 꼭 알려줘.",
    ru: "Хммм, очень интересно. Пожалуйста, сообщи мне, если найдешь новое заклинание.",
};

pub const SPELL_LIST3: Dict<&'static str> = Dict {
    ja: "皆にもその呪文を集めるよう伝えておこう。次からその呪文が店にも並ぶようになるはずだ。",
    en: "I'll tell everyone to collect that spells. It should be available in the shop from now on.",
    zh_cn: "我会告诉大家收集咒语。从现在开始，它应该可以在商店里买到。",
    es: "Le diré a todos que recojan esos hechizos. Debería estar disponible en la tienda a partir de ahora.",
    fr: "Je dirai à tout le monde de collecter ces sorts. Ils devraient être disponibles dans la boutique à partir de maintenant.",
    pt: "Vou dizer a todos para coletarem esses feitiços. Eles devem estar disponíveis na loja a partir de agora.",
    de: "Ich werde allen sagen, dass sie diese Zauber sammeln sollen. Sie sollten ab jetzt im Laden erhältlich sein.",
    ko: "모두에게 그 주문을 모으라고 말할게. 이제부터 그 주문이 가게에 있을 거야.",
    ru: "Я скажу всем собирать эти заклинания. Они должны быть доступны в магазине с этого момента.",
};

pub const SHOP_RABBIT: Dict<&'static str> = Dict {
    ja: "いらっしゃいませ！\n欲しい商品があったら\n持ってきてくださいね",
    en: "Welcome!\nBring me the item\nyou want to buy.",
    zh_cn: "欢迎！\n给我带来\n你想买的商品。",
    es: "¡Bienvenido!\nTráeme el artículo\nque quieres comprar.",
    fr: "Bienvenue!\nApportez-moi l'article\nque vous voulez acheter.",
    pt: "Bem-vindo!\nTraga-me o item\nque você quer comprar.",
    de: "Willkommen!\nBring mir den Artikel,\nden du kaufen möchtest.",
    ko: "어서 오세요!\n사고 싶은 물건을\n가져와 주세요.",
    ru: "Добро пожаловать!\nПринеси мне предмет,\nкоторый хочешь купить.",
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
        pt: format!("Seu total é de {} ouros\nObrigado", golds),
        de: format!("Ihr Gesamtbetrag beträgt {} Gold\nDanke", golds),
        ko: format!("총 {} 골드입니다\n감사합니다", golds),
        ru: format!("Ваш итоговый счет {} золота\nСпасибо", golds),
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
        pt: format!(
            "Ei, ei!\nVocê está {} ouros a menos!\nDevolva o que você não vai comprar",
            golds
        ),
        de: format!(
            "Hey, hey!\nDir fehlen {} Gold!\nGib zurück, was du nicht kaufen wirst",
            golds
        ),
        ko: format!(
            "이봐, 이봐!\n{} 골드가 부족해!\n사지 않을 물건은 돌려놔",
            golds
        ),
        ru: format!(
            "Эй, эй!\nВам не хватает {} золота!\nВерните то, что не будете покупать",
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
    pt: "Ei ei, pague primeiro antes de ir",
    de: "Hey Hey, zuerst bezahlen, bevor du gehst",
    ko: "이봐 이봐, 가기 전에 먼저 지불해",
    ru: "Эй, эй, сначала заплати, прежде чем уйти",
};

// 地名 /////////////////////////////////////////////////////////////////////////////////////////////////////////////

pub const LEVEL0: Dict<&'static str> = Dict {
    ja: "見捨てられた工房",
    en: "Abandoned Workshop",
    zh_cn: "废弃的车间",
    es: "Taller Abandonado",
    fr: "Atelier Abandonné",
    pt: "Oficina Abandonada",
    de: "Verlassene Werkstatt",
    ko: "버려진 작업장",
    ru: "Заброшенная мастерская",
};

pub const LEVEL1: Dict<&'static str> = Dict {
    ja: "図書館跡",
    en: "Library Ruins",
    zh_cn: "图书馆废墟",
    es: "Ruinas de la Biblioteca",
    fr: "Ruines de la Bibliothèque",
    pt: "Ruínas da Biblioteca",
    de: "Bibliotheksruinen",
    ko: "도서관 폐허",
    ru: "Руины библиотеки",
};

pub const LEVEL2: Dict<&'static str> = Dict {
    ja: "地下草原",
    en: "Underground Grassland",
    zh_cn: "地下草原",
    es: "Pradera Subterránea",
    fr: "Prairie Souterraine",
    pt: "Pradaria Subterrânea",
    de: "Unterirdisches Grasland",
    ko: "지하 초원",
    ru: "Подземные луга",
};

pub const LEVEL3: Dict<&'static str> = Dict {
    ja: "古城",
    en: "Ancient Castle",
    zh_cn: "古城",
    es: "Castillo Antiguo",
    fr: "Château Ancien",
    pt: "Castelo Antigo",
    de: "Alte Burg",
    ko: "고대 성",
    ru: "Древний замок",
};

pub const LEVEL4: Dict<&'static str> = Dict {
    ja: "スライムの巣窟",
    en: "Slime Nest",
    zh_cn: "史莱姆巢穴",
    es: "Nido de Slimes",
    fr: "Nid de Slimes",
    pt: "Ninho de Slimes",
    de: "Schleimnest",
    ko: "슬라임 둥지",
    ru: "Гнездо слизней",
};

pub const MULTIPLAY_ARENA: Dict<&'static str> = Dict {
    ja: "対決の洞窟",
    en: "Arena Cave",
    zh_cn: "竞技场洞穴",
    es: "Cueva de la Arena",
    fr: "Grotte de l'Arène",
    pt: "Caverna da Arena",
    de: "Arenahöhle",
    ko: "아레나 동굴",
    ru: "Пещера арены",
};

pub const UNKNOWN_LEVEL: Dict<&'static str> = Dict {
    ja: "不明",
    en: "Unknown",
    zh_cn: "未知",
    es: "Desconocido",
    fr: "Inconnu",
    pt: "Desconhecido",
    de: "Unbekannt",
    ko: "알 수 없음",
    ru: "Неизвестно",
};
