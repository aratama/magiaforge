use crate::language::Dict;

// 一行は日本語で18文字まで

// UI
pub const CLICK_TO_START: Dict<&'static str> = Dict {
    ja: "クリックでスタート",
    en: "Click to Start",
    zh_cn: "点击开始",
    zh_tw: "點擊開始",
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
    zh_tw: "發現的咒語",
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
    zh_tw: "未付款",
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
    zh_tw: "輸入你的名字",
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
    zh_tw: "開始",
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
    zh_tw: "暫停",
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
    zh_tw: "全屏",
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
    zh_tw: "開",
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
    zh_tw: "關",
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
    zh_tw: "背景音樂音量",
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
    zh_tw: "音效音量",
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
    zh_tw: "恢復",
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
    zh_tw: "返回標題",
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
    zh_tw: "排序",
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
    zh_tw: "哦，一個女巫。\n這是我們的商人營地。\n歡迎客人。",
    es: "Oh, una bruja.\nEste es nuestro campamento de comerciantes.\nLos invitados son bienvenidos.",
    fr: "Oh, une sorcière.\nCeci est notre camp de marchands.\nLes invités sont les bienvenus.",
    pt: "Oh, uma bruxa.\nEste é nosso acampamento de mercadores.\nOs convidados são bem-vindos.",
    de: "Oh, eine Hexe.\nDies ist unser Händlerlager.\nGäste sind willkommen.",
    ko: "오, 마녀야.\n여기는 우리 상인 캠프야.\n손님을 환영해.",
    ru: "О, ведьма.\nЭто наш лагерь торговцев.\nГостям рады.",
};

pub const HELLO_RABBITS: Dict<&'static str> = Dict {
    ja: "迷宮に行きたいなら入り口はこの奥だよ。でもよかったら仲間たちにもあいさつしていってくれ。",
    en: "If you want to go to the labyrinth, the entrance is at the back. But if you like, please say hello to my friends.",
    zh_cn: "如果你想去迷宫，入口在后面。但如果你愿意，也请向我的朋友们问好。",
    zh_tw: "如果你想去迷宮，入口在後面。但如果你願意，也請向我的朋友們問好。",
    es: "Si quieres ir al laberinto, la entrada está en la parte trasera. Pero si quieres, por favor saluda a mis amigos.",
    fr: "Si vous voulez aller dans le labyrinthe, l'entrée est à l'arrière. Mais si vous le souhaitez, saluez mes amis.",
    pt: "Se você quiser ir ao labirinto, a entrada é na parte de trás. Mas se quiser, por favor, cumprimente meus amigos.",
    de: "Wenn du in das Labyrinth gehen möchtest, ist der Eingang hinten. Aber wenn du möchtest, grüße bitte meine Freunde.",
    ko: "미로로 가고 싶다면 입구는 뒤에 있어. 하지만 원한다면 내 친구들에게 인사해 주세요.",
    ru: "Если хочешь пойти в лабиринт, вход находится сзади. Но если хочешь, пожалуйста, поздоровайся с моими друзьями.",
};

pub const SINGLEPLAY: Dict<&'static str> = Dict {
    ja: "長い黒髪の魔女だって？うーん、この島でそんな魔女を見かけたことがある気がする。",
    en: "A witch with long black hair? Hmm, I feel like I've seen such a witch on this island.",
    zh_cn: "一个长发黑发的女巫？嗯，我觉得我在这个岛上见过这样的女巫。",
    zh_tw: "一個長發黑發的女巫？嗯，我覺得我在這個島上見過這樣的女巫。",
    es: "¿Una bruja con el pelo largo y negro? Hmm, siento que he visto a una bruja así en esta isla.",
    fr: "Une sorcière aux longs cheveux noirs ? Hmm, je crois avoir vu une telle sorcière sur cette île.",
    pt: "Uma bruxa de cabelos longos e pretos? Hmm, sinto que já vi uma bruxa assim nesta ilha.",
    de: "Eine Hexe mit langen schwarzen Haaren? Hm, ich glaube, ich habe so eine Hexe auf dieser Insel gesehen.",
    ko: "긴 검은 머리의 마녀? 음, 이 섬에서 그런 마녀를 본 적이 있는 것 같아.",
    ru: "Ведьма с длинными черными волосами? Хм, кажется, я видел такую ведьму на этом острове.",
};

pub const SINGLEPLAY_2: Dict<&'static str> = Dict {
    ja: "その魔法陣が地下迷宮の入り口だよ。その探し人が見つかるように幸運を祈るよ。",
    en: "That magic circle is the entrance to the underground labyrinth. I wish you luck in finding the person you're looking for.",
    zh_cn: "那个魔法阵是地下迷宫的入口。祝你好运找到你要找的人。",
    zh_tw: "那個魔法陣是地下迷宮的入口。祝你好運找到你要找的人。",
    es: "Ese círculo mágico es la entrada al laberinto subterráneo. Te deseo suerte en encontrar a la persona que buscas.",
    fr: "Ce cercle magique est l'entrée du labyrinthe souterrain. Je vous souhaite bonne chance pour trouver la personne que vous cherchez.",
    pt: "Esse círculo mágico é a entrada para o labirinto subterrâneo. Desejo-lhe sorte em encontrar a pessoa que procura.",
    de: "Dieser magische Kreis ist der Eingang zum unterirdischen Labyrinth. Ich wünsche dir viel Glück bei der Suche nach der Person, die du suchst.",
    ko: "그 마법진이 지하 미로의 입구야. 찾고 있는 사람을 찾기를 바랄게.",
    ru: "Этот магический круг - вход в подземный лабиринт. Желаю удачи в поисках того, кого ты ищешь.",
};

pub const RESERCH_RABBIT_0: Dict<&'static str> = Dict {
    ja: "昔はこの島にも多くのヒト族がいたらしいが、今は魔法使いが時折訪れるくらいさ。君たち魔法使いはこの地底でいったい何を探しているんだい？",
    en: "There used to be many humans on this island but now only witches occasionally visit. What are you witches looking for in the depths?",
    zh_cn: "这个岛上曾经有很多人类，但现在只有女巫偶尔会来访。你们女巫在深处寻找什么？",
    zh_tw: "這個島上曾經有很多人類，但現在只有女巫偶爾會來訪。你們女巫在深處尋找什麼？",
    es: "Solía haber muchos humanos en esta isla, pero ahora solo las brujas la visitan ocasionalmente. ¿Qué buscan ustedes, las brujas, en las profundidades?",
    fr: "Il y avait autrefois beaucoup d'humains sur cette île, mais maintenant seules les sorcières la visitent occasionnellement. Que cherchez-vous, sorcières, dans les profondeurs?",
    pt: "Costumava haver muitos humanos nesta ilha, mas agora apenas bruxas a visitam ocasionalmente. O que vocês, bruxas, estão procurando nas profundezas?",
    de: "Früher gab es viele Menschen auf dieser Insel, aber jetzt besuchen nur noch gelegentlich Hexen. Was sucht ihr Hexen in den Tiefen?",
    ko: "예전에는 이 섬에 많은 인간이 있었지만 지금은 가끔 마녀들만 방문해. 너희 마녀들은 깊은 곳에서 무엇을 찾고 있니?",
    ru: "Раньше на этом острове было много людей, но теперь его изредка посещают только ведьмы. Что вы, ведьмы, ищете в глубинах?",
};

pub const RESERCH_RABBIT_1: Dict<&'static str> = Dict {
    ja: "ふうん、君は黒髪の魔女を探しているのか。たしか半年ほど前にそんな魔女がキャンプを訪れたな。それが君の探し人かはわからないが。",
    en: "Hmm, you're looking for a witch with black hair. I remember a witch like that visited the camp about half a year ago. I don't know if that's the person you're looking for.",
    zh_cn: "嗯，你在找一个黑发女巫。我记得大约半年前有一个这样的女巫来过营地。我不知道那是否是你要找的人。",
    zh_tw: "嗯，你在找一個黑髮女巫。我記得大約半年前有一個這樣的女巫來過營地。我不知道那是否是你要找的人。",
    es: "Hmm, estás buscando a una bruja con el pelo negro. Recuerdo que una bruja así visitó el campamento hace unos seis meses. No sé si es la persona que estás buscando.",
    fr: "Hmm, vous cherchez une sorcière aux cheveux noirs. Je me souviens qu'une sorcière comme ça a visité le camp il y a environ six mois. Je ne sais pas si c'est la personne que vous cherchez.",
    pt: "Hmm, você está procurando uma bruxa de cabelo preto. Eu me lembro de uma bruxa assim ter visitado o acampamento há cerca de seis meses. Não sei se é a pessoa que você está procurando.",
    de: "Hmm, du suchst eine Hexe mit schwarzen Haaren. Ich erinnere mich, dass vor etwa einem halben Jahr eine solche Hexe das Lager besucht hat. Ich weiß nicht, ob das die Person ist, die du suchst.",
    ko: "흠, 검은 머리의 마녀를 찾고 있구나. 약 반년 전에 그런 마녀가 캠프를 방문한 적이 있어. 네가 찾는 사람이 맞는지는 모르겠어.",
    ru: "Хм, ты ищешь ведьму с черными волосами. Помню, такая ведьма посещала лагерь около полугода назад. Не знаю, та ли это, кого ты ищешь.",
};

pub const RESERCH_RABBIT_2: Dict<&'static str> = Dict {
    ja: "その魔女は迷宮に入っていった。それからはずっと姿を見ていない。まあ迷宮の中を探してみるほかないだろうな。",
    en: "That witch went into the labyrinth. I haven't seen her since. You'll have to search the labyrinth.",
    zh_cn: "那个女巫进入了迷宫。从那以后我就没见过她。你得在迷宫里找找。",
    zh_tw: "那個女巫進入了迷宮。從那以後我就沒見過她。你得在迷宮裡找找。",
    es: "Esa bruja entró en el laberinto. No la he visto desde entonces. Tendrás que buscar en el laberinto.",
    fr: "Cette sorcière est entrée dans le labyrinthe. Je ne l'ai pas vue depuis. Vous devrez chercher dans le labyrinthe.",
    pt: "Aquela bruxa entrou no labirinto. Não a vi desde então. Você terá que procurar no labirinto.",
    de: "Diese Hexe ging in das Labyrinth. Ich habe sie seitdem nicht mehr gesehen. Du musst im Labyrinth suchen.",
    ko: "그 마녀는 미로로 들어갔어. 그 이후로 그녀를 본 적이 없어. 미로에서 찾아봐야 할 거야.",
    ru: "Та ведьма пошла в лабиринт. С тех пор я ее не видел. Тебе придется искать в лабиринте.",
};

pub const RESERCH_RABBIT_3: Dict<&'static str> = Dict {
    ja: "ところで、キャンプにどこからかニワトリが入り込んでいて鬱陶しいな。鳴き声がうるさくて研究に差し支える",
    en: "By the way, a chicken has somehow gotten into the camp. Its clucking is annoying and it's disturbing my research.",
    zh_cn: "顺便说一下，一只鸡不知怎么进了营地。它的咯咯叫声很烦人，打扰了我的研究。",
    zh_tw: "順便說一下，一隻雞不知怎麼進了營地。它的咯咯叫聲很煩人，打擾了我的研究。",
    es: "Por cierto, una gallina se ha metido en el campamento. Su cacareo es molesto y está perturbando mi investigación.",
    fr: "Au fait, une poule est entrée dans le camp. Son caquètement est agaçant et perturbe mes recherches.",
    pt: "A propósito, uma galinha entrou no acampamento. Seu cacarejo é irritante e está atrapalhando minha pesquisa.",
    de: "Übrigens, ein Huhn hat sich irgendwie ins Lager geschlichen. Sein Gackern ist nervig und stört meine Forschung.",
    ko: "그런데 닭 한 마리가 캠프에 들어왔어. 그 울음소리가 짜증나고 내 연구를 방해하고 있어.",
    ru: "Кстати, в лагерь как-то пробралась курица. Ее кудахтанье раздражает и мешает моим исследованиям.",
};

pub const RESERCH_RABBIT_4: Dict<&'static str> = Dict {
    ja: "ぼくらがニワトリを飼っているわけじゃない。あいつらはぼくらの食料を狙って勝手に入り込んでいるんだ。",
    en: "We don't keep chickens. They sneak in to steal our food.",
    zh_cn: "我们不养鸡。它们偷偷进来偷我们的食物。",
    zh_tw: "我們不養雞。牠們偷偷進來偷我們的食物。",
    es: "No criamos pollos. Se cuelan para robar nuestra comida.",
    fr: "Nous n'élevons pas de poulets. Ils se faufilent pour voler notre nourriture.",
    pt: "Nós não criamos galinhas. Elas entram sorrateiramente para roubar nossa comida.",
    de: "Wir halten keine Hühner. Sie schleichen sich ein, um unser Essen zu stehlen.",
    ko: "우리는 닭을 키우지 않아. 그들은 우리의 음식을 훔치기 위해 몰래 들어와.",
    ru: "Мы не держим кур. Они проникают, чтобы украсть нашу еду.",
};

pub const RESERCH_RABBIT_5: Dict<&'static str> = Dict {
    ja: "ぼくらはニワトリなんて食べないよ。むしろニワトリがぼくらを食べようとしてるのさ。",
    en: "We don't eat chickens. Rather, the chickens are trying to eat us.",
    zh_cn: "我们不吃鸡。相反，鸡在试图吃我们。",
    zh_tw: "我們不吃雞。相反，雞在試圖吃我們。",
    es: "No comemos pollos. Más bien, los pollos están tratando de comernos.",
    fr: "Nous ne mangeons pas de poulets. Au contraire, ce sont les poulets qui essaient de nous manger.",
    pt: "Nós não comemos frango. Na verdade, os frangos estão tentando nos comer.",
    de: "Wir essen keine Hühner. Vielmehr versuchen die Hühner, uns zu essen.",
    ko: "우리는 닭을 먹지 않아. 오히려 닭들이 우리를 먹으려고 해.",
    ru: "Мы не едим кур. Скорее, куры пытаются съесть нас.",
};

pub const MULTIPLAY: Dict<&'static str> = Dict {
    ja: "この先はマルチプレイ用レベルだよ。\n気軽に遊んでいってね。\n誰かいるかはわからないけど。",
    en: "It seems that this is a level for multiplayer.\nFeel free to play.\nI don't know if anyone is there.",
    zh_cn: "这似乎是一个多人游戏的级别。\n随意玩。\n我不知道有没有人在那里。",
    zh_tw: "這似乎是一個多人遊戲的級別。\n隨意玩。\n我不知道有沒有人在那裡。",
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
    zh_tw: "你想變強嗎？\n在這裡練習是很好的。\n沙袋們會成為你的對手。",
    es: "¿Quieres ser fuerte?\nEs bueno practicar aquí.\nLos sacos de arena serán tus oponentes.",
    fr: "Voulez-vous devenir fort?\nC'est bien de s'entraîner ici.\nLes sacs de sable seront vos adversaires.",
    pt: "Você quer ser forte?\nÉ bom praticar aqui.\nOs sacos de areia serão seus oponentes.",
    de: "Willst du stark werden?\nEs ist gut, hier zu üben.\nDie Sandsäcke werden deine Gegner sein.",
    ko: "강해지고 싶어?\n여기서 연습하는 것이 좋아.\n샌드백들이 상대가 되어줄 거야.",
    ru: "Хочешь стать сильным?\nЗдесь хорошо тренироваться.\nМешки с песком будут твоими противниками.",
};

pub const TRAINING_RABBIT_1: Dict<&'static str> = Dict {
    ja: "ところで最近、地下迷宮にとても大きなスライムが現れてね。大暴れして困っているんだ。",
    en: "Lately, a huge slime has appeared in the labyrinth. It's causing a lot of trouble.",
    zh_cn: "最近，一个巨大的史莱姆出现在迷宫里。它造成了很多麻烦。",
    zh_tw: "最近，一個巨大的史萊姆出現在迷宮裡。它造成了很多麻煩。",
    es: "Últimamente, un gran slime ha aparecido en el laberinto. Está causando muchos problemas.",
    fr: "Dernièrement, un énorme slime est apparu dans le labyrinthe. Il cause beaucoup de problèmes.",
    pt: "Ultimamente, um grande slime apareceu no labirinto. Está causando muitos problemas.",
    de: "In letzter Zeit ist ein riesiger Schleim im Labyrinth aufgetaucht. Es verursacht viele Probleme.",
    ko: "최근에 미로에 거대한 슬라임이 나타났어. 많은 문제를 일으키고 있어.",
    ru: "В последнее время в лабиринте появился огромный слизень. Он вызывает много проблем.",
};

pub const TRAINING_RABBIT_2: Dict<&'static str> = Dict {
    ja: "なにしろぼくらは地下迷宮で遺物を拾って生計を立てているからね。あんなのがうろついていたら落ち着いて探索もできやしない。",
    en: "After all, we make a living by picking up relics in the labyrinth. If such a thing is wandering around, we can't explore calmly.",
    zh_cn: "毕竟，我们是靠在迷宫里捡遗物谋生的。如果这样的东西四处游荡，我们就无法平静地探索。",
    zh_tw: "畢竟，我們是靠在迷宮裡撿遺物謀生的。如果這樣的東西四處遊蕩，我們就無法平靜地探索。",
    es: "Después de todo, nos ganamos la vida recogiendo reliquias en el laberinto. Si algo así está deambulando, no podemos explorar con calma.",
    fr: "Après tout, nous gagnons notre vie en ramassant des reliques dans le labyrinthe. Si une telle chose erre, nous ne pouvons pas explorer calmement.",
    pt: "Afinal, ganhamos a vida recolhendo relíquias no labirinto. Se algo assim estiver vagando, não podemos explorar calmamente.",
    de: "Schließlich verdienen wir unseren Lebensunterhalt, indem wir Relikte im Labyrinth aufsammeln. Wenn so etwas herumwandert, können wir nicht ruhig erkunden.",
    ko: "결국 우리는 미로에서 유물을 주워 생계를 유지해. 그런 것이 돌아다니면 우리는 차분하게 탐험할 수 없어.",
    ru: "В конце концов, мы зарабатываем на жизнь, собирая реликвии в лабиринте. Если такое существо будет бродить вокруг, мы не сможем спокойно исследовать.",
};

pub const TRAINING_RABBIT_3: Dict<&'static str> = Dict {
    ja: "あなたがあのスライムを討伐してくれたら、とても助かるんだけど。",
    en: "If you could defeat that slime, I would be very grateful.",
    zh_cn: "如果你能打败那个史莱姆，我会非常感激。",
    zh_tw: "如果你能打敗那個史萊姆，我會非常感激。",
    es: "Si pudieras derrotar a ese slime, estaría muy agradecido.",
    fr: "Si vous pouviez vaincre ce slime, je vous en serais très reconnaissant.",
    pt: "Se você pudesse derrotar aquele slime, eu ficaria muito grato.",
    de: "Wenn du diesen Schleim besiegen könntest, wäre ich dir sehr dankbar.",
    ko: "네가 그 슬라임을 물리쳐 준다면 정말 고마울 거야.",
    ru: "Если бы ты мог победить этого слизня, я был бы очень благодарен.",
};

pub const TRAINING_RABBIT_4: Dict<&'static str> = Dict {
    ja: "その大きなスライムは体当たりで攻撃してくるが、足が早ければ逃げるのは難しくない。",
    en: "The huge slime attacks with a body blow, but if you have fast legs, it's not hard to escape.",
    zh_cn: "巨大的史莱姆会用身体冲击攻击，但如果你的腿很快",
    zh_tw: "巨大的史萊姆會用身體衝擊攻擊，但如果你的腿很快",
    es: "El gran slime ataca con un golpe de cuerpo, pero si tienes piernas rápidas, no es difícil escapar.",
    fr: "Le gros slime attaque avec un coup de corps, mais si vous avez des jambes rapides, il n'est pas difficile de s'échapper.",
    pt: "O grande slime ataca com um golpe corporal, mas se você tiver pernas rápidas, não é difícil escapar.",
    de: "Der riesige Schleim greift mit einem Körperstoß an, aber wenn du schnelle Beine hast, ist es nicht schwer zu entkommen.",
    ko: "거대한 슬라임은 몸통 박치기로 공격하지만, 다리가 빠르면 도망치는 것은 어렵지 않아.",
    ru: "Огромный слизень атакует телесным ударом, но если у тебя быстрые ноги, убежать несложно.",
};

pub const TRAINING_RABBIT_5: Dict<&'static str> = Dict {
    ja: "それと、あいつは仲間の小さなスライムを呼び寄せるんだ。囲まれると逃げ道を失う。周囲のスライムは素早く倒したほうがいい。",
    en: "And it calls small slimes to its side. If you are surrounded, you will lose your escape route. It's better to defeat the surrounding slimes quickly.",
    zh_cn: "它会召唤小史莱姆来帮忙。如果你被包围，你就会失去逃跑的路线。最好快速击败周围的史莱姆。",
    zh_tw: "它會召喚小史萊姆來幫忙。如果你被包圍，你就會失去逃跑的路線。最好快速擊敗周圍的史萊姆。",
    es: "Y llama a pequeños slimes a su lado. Si estás rodeado, perderás tu ruta de escape. Es mejor derrotar rápidamente a los slimes circundantes.",
    fr: "Et il appelle des petits slimes à ses côtés. Si vous êtes entouré, vous perdrez votre route d'évasion. Il vaut mieux vaincre rapidement les slimes environnants.",
    pt: "E ele chama pequenos slimes para o seu lado. Se você estiver cercado, perderá sua rota de fuga. É melhor derrotar rapidamente os slimes ao redor.",
    de: "Und es ruft kleine Schleime zu sich. Wenn du umzingelt bist, verlierst du deinen Fluchtweg. Es ist besser, die umliegenden Schleime schnell zu besiegen.",
    ko: "그리고 작은 슬라임들을 불러들여. 둘러싸이면 도망갈 길을 잃게 돼. 주변의 슬라임들을 빨리 물리치는 것이 좋아.",
    ru: "И он зовет к себе маленьких слизней. Если ты окажешься окружен, ты потеряешь путь к бегству. Лучше быстро победить окружающих слизней.",
};

pub const SPELL_LIST1: Dict<&'static str> = Dict {
    ja: "私は魔法使いたちの操る呪文に興味があってね。君の知っている呪文について教えてくれないか？",
    en: "I'm interested in the spells cast by witches. Can you tell me about the spells you know?",
    zh_cn: "我对女巫施展的咒语很感兴趣。你能告诉我你所知道的咒语吗？",
    zh_tw: "我對女巫施展的咒語很感興趣。你能告訴我你所知道的咒語嗎？",
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
    zh_tw: "嗯嗯，非常有趣。如果您發現新咒語，請告訴我。",
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
    zh_tw: "我會告訴大家收集咒語。從現在開始，它應該可以在商店裡買到。",
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
    zh_tw: "歡迎！\n給我帶來\n你想買的商品。",
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
        zh_tw: format!("您的總點數為 {} 枚金牌。謝謝你！", golds),
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
        zh_tw: format!("嘿，嘿！\n你少了{}枚金牌！\n把你不買的東西放回去", golds),
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
    zh_tw: "嘿，嘿，先付錢再走",
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
    zh_tw: "廢棄的車間",
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
    zh_tw: "圖書館廢墟",
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
    zh_tw: "地下草原",
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
    zh_tw: "古城",
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
    zh_tw: "史萊姆巢穴",
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
    zh_tw: "競技場洞穴",
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
    zh_tw: "未知",
    es: "Desconocido",
    fr: "Inconnu",
    pt: "Desconhecido",
    de: "Unbekannt",
    ko: "알 수 없음",
    ru: "Неизвестно",
};
