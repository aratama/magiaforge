function* GuideRabbit(context) {

    yield {
        type: "Speech",
        ja: "テスト",
        en: "",
        zh_cn: "",
        zh_tw: "",
        es: "",
        fr: "",
        pt: "",
        de: "",
        ko: "",
        ru: "",
    };

    yield {
        type: "Wait",
        count: 60,
    };

    yield {
        type: "Speech",
        ja: "テスト2",
        en: "",
        zh_cn: "",
        zh_tw: "",
        es: "",
        fr: "",
        pt: "",
        de: "",
        ko: "",
        ru: "",
    };

    yield {
        type: "Speech",
        ja: "おや、きみは魔法使いだね。ここはぼくらの商人キャンプだよ。来客は歓迎さ。",
        en: "Oh, you are a mage. This is our merchant camp. Visitors are welcome.",
        zh_cn: "哦，你是一个法师。这是我们的商人营地。欢迎来访。",
        zh_tw: "哦，你是一個法師。這是我們的商人營地。歡迎來訪。",
        es: "Oh, eres un mago. Este es nuestro campamento de comerciantes. Los visitantes son bienvenidos.",
        fr: "Oh, vous êtes un mage. C'est notre camp de marchands. Les visiteurs sont les bienvenus.",
        pt: "Oh, você é um mago. Este é o nosso acampamento de comerciantes. Visitantes são bem-vindos.",
        de: "Oh, du bist ein Magier. Dies ist unser Händlerlager. Besucher sind willkommen.",
        ko: "오, 당신은 마법사군요. 여기는 우리의 상인 캠프입니다. 방문객을 환영합니다.",
        ru: "О, вы маг. Это наш лагерь торговцев. Посетители приветствуются.",
    };
    yield {
        type: "Speech",
        ja: "通りすがりに大鴉に襲われた？それは災難だったね。",
        en: "You were attacked by a large raven on the way? That's unfortunate.",
        zh_cn: "你在路上被一只大乌鸦袭击了？那真是不幸。",
        zh_tw: "你在路上被一隻大烏鴉襲擊了？那真是不幸。",
        es: "¿Fuiste atacado por un gran cuervo en el camino aquí? Eso es desafortunado.",
        fr: "Vous avez été attaqué par un grand corbeau en venant ici ? C'est malheureux.",
        pt: "Você foi atacado por um grande corvo no caminho para cá? Isso é lamentável.",
        de: "Wurden Sie auf dem Weg hierher von einem großen Raben angegriffen? Das ist bedauerlich.",
        ko: "여기 오는 길에 큰 까마귀에게 공격당했어? 그것 참 불운하군.",
        ru: "Тебя по пути сюда атаковал большой ворон? Это неудача.",
    };
    yield {
        type: "Speech",
        ja: "おそらくはこの島に住む『漆黒の魔女』の使い魔のしわざだろう。",
        en: "It is probably the work of the 'Witch of Jet Black' who lives on this island.",
        zh_cn: "这可能是住在这个岛上的“漆黑女巫”的杰作。",
        zh_tw: "這可能是住在這個島上的“漆黑女巫”的傑作。",
        es: "Probablemente sea obra de la 'Bruja de Azabache' que vive en esta isla.",
        fr: "C'est probablement l'œuvre de la 'Sorcière de Jais' qui vit sur cette île.",
        pt: "Provavelmente é obra da 'Bruxa de Ébano' que vive nesta ilha.",
        de: "Es ist wahrscheinlich das Werk der 'Hexe von Pechschwarz', die auf dieser Insel lebt.",
        ko: "아마도 이 섬에 사는 '칠흑의 마녀'의 짓일 거야.",
        ru: "Вероятно, это дело рук 'Ведьмы Черного Яшмы', которая живет на этом острове.",
    };
    yield {
        type: "Speech",
        ja: "我々では折れた魔法の箒の修理はできないな。自分で直してもらうしかない。",
        en: "We can't repair your broken magic broom. You'll have to fix it yourself.",
        zh_cn: "我们无法修理你破损的魔法扫帚。你得自己修理。",
        zh_tw: "我們無法修理你破損的魔法掃帚。你得自己修理。",
        es: "No podemos reparar tu escoba mágica rota. Tendrás que arreglarla tú mismo.",
        fr: "Nous ne pouvons pas réparer votre balai magique cassé. Vous devrez le réparer vous-même.",
        pt: "Não podemos consertar sua vassoura mágica quebrada. Você terá que consertá-la sozinho.",
        de: "Wir können deinen kaputten magischen Besen nicht reparieren. Du musst ihn selbst reparieren.",
        ko: "우리는 당신의 부러진 마법 빗자루를 고칠 수 없습니다. 당신이 직접 고쳐야 합니다.",
        ru: "Мы не можем починить вашу сломанную волшебную метлу. Вам придется починить ее самостоятельно."
    };
    yield {
        type: "Speech",
        ja: "この島の迷宮に行けば、修理の役に立つ呪文があるかもしれない。迷宮の入り口はこの奥だよ。",
        en: "If you go to the labyrinth on this island, you might find a spell that can help with the repair. The entrance to the labyrinth is further in.",
        zh_cn: "如果你去这个岛上的迷宫，你可能会找到一个可以帮助修理的咒语。迷宫的入口在里面。",
        zh_tw: "如果你去這個島上的迷宮，你可能會找到一個可以幫助修理的咒語。迷宮的入口在裡面。",
        es: "Si vas al laberinto en esta isla, podrías encontrar un hechizo que pueda ayudar con la reparación. La entrada al laberinto está más adentro.",
        fr: "Si vous allez dans le labyrinthe de cette île, vous pourriez trouver un sort qui peut aider à la réparation. L'entrée du labyrinthe est plus loin.",
        pt: "Se você for ao labirinto nesta ilha, pode encontrar um feitiço que pode ajudar no reparo. A entrada do labirinto está mais adiante.",
        de: "Wenn du in das Labyrinth auf dieser Insel gehst, könntest du einen Zauber finden, der bei der Reparatur hilft. Der Eingang zum Labyrinth ist weiter drinnen.",
        ko: "이 섬의 미로에 가면 수리에 도움이 될 수 있는 주문을 찾을 수 있을 거야. 미로의 입구는 더 안쪽에 있어.",
        ru: "Если ты пойдешь в лабиринт на этом острове, возможно, найдешь заклинание, которое поможет с ремонтом. Вход в лабиринт находится дальше.",
    };
    yield {
        type: "Speech",
        ja: "迷宮に行く前に、ぼくらの店に立ち寄るといい。迷宮で拾った呪文を売っているんだ。",
        en: "Before going to the labyrinth, you should stop by our shop. We sell spells that we found in the labyrinth.",
        zh_cn: "在去迷宫之前，你应该先去我们的商店。我们出售在迷宫中找到的咒语。",
        zh_tw: "在去迷宮之前，你應該先去我們的商店。我們出售在迷宮中找到的咒語。",
        es: "Antes de ir al laberinto, deberías pasar por nuestra tienda. Vendemos hechizos que encontramos en el laberinto.",
        fr: "Avant d'aller dans le labyrinthe, vous devriez passer par notre boutique. Nous vendons des sorts que nous avons trouvés dans le labyrinthe.",
        pt: "Antes de ir ao labirinto, você deve passar pela nossa loja. Vendemos feitiços que encontramos no labirinto.",
        de: "Bevor du in das Labyrinth gehst, solltest du in unserem Laden vorbeischauen. Wir verkaufen Zauber, die wir im Labyrinth gefunden haben.",
        ko: "미로에 가기 전에 우리 가게에 들르는 것이 좋아. 우리는 미로에서 발견한 주문을 팔고 있어.",
        ru: "Перед тем как идти в лабиринт, тебе стоит заглянуть в наш магазин. Мы продаем заклинания, которые нашли в лабиринте.",
    };
    if (!inventory.includes("Lantern")) {
        yield {
            type: "Speech",
            ja: "そうそう。この辺りは薄暗い。これを持っていくといい。",
            en: "Right, it's dark around here. You should take this.",
            zh_cn: "对了，这里很暗。你应该带上这个。",
            zh_tw: "對了，這裡很暗。你應該帶上這個。",
            es: "Cierto, está oscuro por aquí. Deberías llevar esto.",
            fr: "C'est vrai, il fait sombre par ici. Vous devriez prendre ceci.",
            pt: "É verdade, está escuro por aqui. Você deveria levar isto.",
            de: "Richtig, es ist dunkel hier. Du solltest das mitnehmen.",
            ko: "그래, 이 근처는 어두워. 이걸 가져가는 게 좋을 거야.",
            ru: "Верно, здесь темно. Тебе стоит взять это."
        };
        yield {
            type: "Close",
        };
        yield {
            type: "GetSpell",
            spell: "Lantern",
        };
    }
}


function* SingleplayRabbit() {
    yield {
        type: "Speech",
        ja: "その魔法陣が地下迷宮の入り口だよ。探しものが見つかるよう、幸運を祈るよ。",
        en: "That magic circle is the entrance to the underground labyrinth. I wish you luck in finding what you're looking for.",
        zh_cn: "那个魔法阵是地下迷宫的入口。祝你好运找到你要找的东西。",
        zh_tw: "那個魔法陣是地下迷宮的入口。祝你好運找到你要找的東西。",
        es: "Ese círculo mágico es la entrada al laberinto subterráneo. Te deseo suerte en encontrar lo que buscas.",
        fr: "Ce cercle magique est l'entrée du labyrinthe souterrain. Je vous souhaite bonne chance pour trouver ce que vous cherchez.",
        pt: "Esse círculo mágico é a entrada para o labirinto subterrâneo. Desejo-lhe sorte em encontrar o que procura.",
        de: "Dieser magische Kreis ist der Eingang zum unterirdischen Labyrinth. Ich wünsche dir viel Glück bei der Suche nach dem, was du suchst.",
        ko: "그 마법진이 지하 미로의 입구야. 찾고 있는 것을 찾기를 바랄게.",
        ru: "Этот магический круг - вход в подземный лабиринт. Желаю удачи в поисках того, что ты ищешь.",
    };
    yield {
        type: "Speech",
        ja: "そうそう、最近迷宮の奥に大きなスライムが現れるようになってね。気を付けたほうがいいよ。",
        en: "By the way, a huge slime has recently appeared deep in the labyrinth. You should be careful.",
        zh_cn: "顺便说一下，最近在迷宫深处出现了一个巨大的史莱姆。你应该小心。",
        zh_tw: "順便說一下，最近在迷宮深處出現了一個巨大的史萊姆。你應該小心。",
        es: "Por cierto, recientemente ha aparecido un gran slime en lo profundo del laberinto. Deberías tener cuidado.",
        fr: "Au fait, un énorme slime est récemment apparu au fond du labyrinthe. Vous devriez faire attention.",
        pt: "A propósito, recentemente apareceu um grande slime no fundo do labirinto. Você deve ter cuidado.",
        de: "Übrigens, tief im Labyrinth ist kürzlich ein riesiger Schleim aufgetaucht. Du solltest vorsichtig sein.",
        ko: "그런데 최근에 미로 깊숙한 곳에 거대한 슬라임이 나타났어. 조심하는 게 좋아.",
        ru: "Кстати, недавно в глубине лабиринта появился огромный слизень. Тебе следует быть осторожным.",
    };
}
function* MultiplayerRabbit() {
    yield {
        type: "Speech",
        ja: "この先はマルチプレイ用レベルだよ。気軽に遊んでいってね。誰かいるかはわからないけど。",
        en: "This is a multiplayer level. Feel free to play. I don't know if anyone is here though.",
        zh_cn: "这是一个多人游戏关卡。随便玩吧。不过我不知道有没有人在这里。",
        zh_tw: "這是一個多人遊戲關卡。隨便玩吧。不過我不知道有沒有人在這裡。",
        es: "Este es un nivel multijugador. Siéntete libre de jugar. No sé si hay alguien aquí, sin embargo.",
        fr: "Ceci est un niveau multijoueur. N'hésitez pas à jouer. Je ne sais pas si quelqu'un est là, cependant.",
        pt: "Este é um nível multijogador. Sinta-se à vontade para jogar. Não sei se há alguém aqui, no entanto.",
        de: "Dies ist ein Mehrspieler-Level. Fühlen Sie sich frei zu spielen. Ich weiß jedoch nicht, ob jemand hier ist.",
        ko: "이곳은 멀티플레이 레벨입니다. 마음껏 플레이하세요. 여기에 누가 있는지는 모르겠어요.",
        ru: "Это многопользовательский уровень. Играйте на здоровье. Не знаю, есть ли здесь кто-нибудь, правда.",
    };
}
function* ReadingRabbit() {
    yield {
        type: "Speech",
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
    yield {
        type: "Speech",
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
    yield {
        type: "Speech",
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
    yield {
        type: "Speech",
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
    yield {
        type: "Speech",
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
    yield {
        type: "Speech",
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
}

function* TrainingRabbit() {
    yield {
        type: "Speech",
        ja: "キミも訓練していくかい？サンドバッグくんたちが相手になってくれる。",
        en: "Would you like to train? The sandbags will be your opponents.",
        zh_cn: "你想训练吗？沙袋会是你的对手。",
        zh_tw: "你想訓練嗎？沙袋會是你的對手。",
        es: "¿Te gustaría entrenar? Los sacos de arena serán tus oponentes.",
        fr: "Voulez-vous vous entraîner ? Les sacs de sable seront vos adversaires.",
        pt: "Você gostaria de treinar? Os sacos de areia serão seus oponentes.",
        de: "Möchtest du trainieren? Die Sandsäcke werden deine Gegner sein.",
        ko: "훈련하고 싶니? 샌드백이 상대가 되어줄 거야.",
        ru: "Хочешь потренироваться? Мешки с песком будут твоими противниками.",
    };
    yield {
        type: "Speech",
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
    yield {
        type: "Speech",
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
    yield {
        type: "Speech",
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
    yield {
        type: "Speech",
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
    yield {
        type: "Speech",
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
}
function* SpellListRabbit() {
    spellListOpen = true;
    yield {
        type: "Speech",
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
    yield {
        type: "Speech",
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
    yield {
        type: "Speech",
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
    }
    spellListOpen = false;
}

function* ShopRabbit() {
    yield {
        type: "Speech",
        ja: "いらっしゃい！迷宮で拾った遺物を売っているんだ。",
        en: "Welcome! We sell relics found in the labyrinth.",
        zh_cn: "欢迎光临！我们出售在迷宫里找到的遗物。",
        zh_tw: "歡迎光臨！我們出售在迷宮裡找到的遺物。",
        es: "¡Bienvenido! Vendemos reliquias encontradas en el laberinto.",
        fr: "Bienvenue ! Nous vendons des reliques trouvées dans le labyrinthe.",
        pt: "Bem-vindo! Vendemos relíquias encontradas no labirinto.",
        de: "Willkommen! Wir verkaufen Relikte, die im Labyrinth gefunden wurden.",
        ko: "어서오세요! 우리는 미로에서 발견한 유물을 판매하고 있어요.",
        ru: "Добро пожаловать! Мы продаем реликвии, найденные в лабиринте."
    };
}

function* HugeSlimeDespawn() {
    yield {
        type: "Sprite",
        name: "huge slime body",
        position: actor_position,
        aseprite: "enemy/huge_slime.aseprite"
    };
    yield { type: "BGM", path: null };
    yield { type: "Shake", value: 6.0, attenuation: -0.5 };
    yield { type: "SE", path: "audio/雷魔法4.ogg" };
    yield {
        type: "Flash",
        position: actor_position,
        intensity: 10.0,
        radius: 480.0,
        duration: 10,
        reverse: false,
    };
    yield { type: "Wait", count: 60 };
    yield { type: "Shake", value: 6.0, attenuation: -0.5 };
    yield { type: "SE", path: "audio/雷魔法4.ogg" };
    yield {
        type: "Flash",
        position: actor_position,
        intensity: 10.0,
        radius: 480.0,
        duration: 10,
        reverse: false,
    };
    yield { type: "Wait", count: 180 };
    yield { type: "Shake", value: 6.0, attenuation: 0.0 };
    yield { type: "SE", path: "audio/地震魔法2.ogg" };
    yield {
        type: "Flash",
        position: actor_position,
        intensity: 10.0,
        radius: 240.0,
        duration: 240,
        reverse: true,
    };
    yield { type: "Wait", count: 240 };
    yield { type: "SE", path: "audio/雷魔法4.ogg" };
    yield {
        type: "Flash",
        position: actor_position,
        intensity: 10.0,
        radius: 240.0,
        duration: 240,
        reverse: false,
    };
    yield { type: "Despawn", name: "huge slime body" };
    yield { type: "Shake", value: 6.0, attenuation: -0.5 };
    // yield {
    //     type: "Wait",
    //     count: 240
    // };
    // yield {
    //     type: "SetTile",
    //     x: 22,
    //     y: 153,
    //     w: 5,
    //     h: 5,
    //     tile: "StoneTile",
    // };
    // yield { type: "SE", path: "audio/kuzureru.ogg" };
    // yield {
    //     type: "SpawnRaven",
    //     name: "raven",
    //     position: [392.0, -2504.0]
    // };
    // yield { type: "Wait", count: 120 };
    // yield { type: "SetCameraTarget", name: "raven" };
    // yield { type: "Wait", count: 120 };
    // yield { type: "SetCameraTarget", name: null };
    // yield { type: "Despawn", name: "raven" };
}