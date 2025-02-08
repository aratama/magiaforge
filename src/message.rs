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

pub const NEW_SPELL: Dict<&'static str> = Dict {
    ja: "新しい呪文を発見！",
    en: "New Spell Discovered!",
    zh_cn: "发现新咒语！",
    zh_tw: "發現新咒語！",
    es: "¡Nuevo Hechizo Descubierto!",
    fr: "Nouveau Sort Découvert !",
    pt: "Novo Feitiço Descoberto!",
    de: "Neuer Zauber Entdeckt!",
    ko: "새로운 주문 발견!",
    ru: "Новое заклинание обнаружено!",
};

/// オープニングデモを飛ばすボタンのラベルテキスト
pub const SKIP: Dict<&'static str> = Dict {
    ja: "スキップ",
    en: "Skip",
    zh_cn: "跳过",
    zh_tw: "跳過",
    es: "Saltar",
    fr: "Passer",
    pt: "Pular",
    de: "Überspringen",
    ko: "건너뛰기",
    ru: "Пропустить",
};
