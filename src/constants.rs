// Validation for various collections
pub const EXPECTED_ITEMS: u32 = 123;
pub const EXPECTED_LOCATIONS: u32 = 134;
pub const EXPECTED_STRINGS_RESPONSES: u32 = 174;
pub const EXPECTED_STRINGS_PUZZLES: u32 = 24;

// Scoring stuff
pub const SCORE_PUZZLE: u32 = 20; // The score the player gets for every puzzle solved
pub const SCORE_TREASURE: u32 = 10; // The score the player gets for each treasure stowed
pub const PENALTY_DEATH: u32 = 25; // The value deducted from player's score for every death
pub const PENALTY_HINT: u32 = 10; // The value deducted from player's score for every hint they request

// Turn bounds
pub const MAX_MOVES_EVENT: u32 = 150; // Random events will all be printed by the time this number of instructions entered
pub const MIN_MOVES_EVENT: u32 = 15; // Random events will not be printed before this number of instructions entered

// Death stuff
pub const DEATH_DIVISOR_NORMAL: u32 = 4;
pub const DEATH_DIVISOR_SMASHED: u32 = 1;

// ID numbers of specific locations
pub const LOCATION_ID_AIRLOCKE: u32 = 31; // The airlock just off the Recreation Hub
pub const LOCATION_ID_AIRLOCKEOUT: u32 = 36; // The area immediately outside Airlock East
pub const LOCATION_ID_ANTEROOM: u32 = 125; // The room beneath the observatory
pub const LOCATION_ID_CELLAR: u32 = 29; // The only cellar
pub const LOCATION_ID_CHECKPOINT: u32 = 32; // The security checkpoint between the Recreation Hub and the Control Hub
pub const LOCATION_ID_DOCKING: u32 = 21; // Where the pirate ship will dock
pub const LOCATION_ID_DOCKINGCONTROL: u32 = 19; // The docking control area
pub const LOCATION_ID_GARDEN: u32 = 27; // The garden, at the foot of the tree
pub const LOCATION_ID_GRAVEYARD: u32 = 0; // Where retired items go
pub const LOCATION_ID_INVENTORY: u32 = 1; // Dummy value
pub const LOCATION_ID_NURSERY: u32 = 3; // Default location of items before they appear after solving puzzles etc.
pub const LOCATION_ID_REFLECTION: u32 = 120; // The mirror room
pub const LOCATION_ID_SHIP: u32 = 49; // Inside the pirate ship
pub const LOCATION_ID_SHUTTLE: u32 = 23; // Inside the shuttle
pub const LOCATION_ID_SLEEP_0: u32 = 91; // Cot room
pub const LOCATION_ID_SLEEP_1: u32 = 92; // Start of the dream
pub const LOCATION_ID_SLEEP_2: u32 = 97; // End of the dream
pub const LOCATION_ID_THOR: u32 = 106; // Thor room
pub const LOCATION_ID_TREASURESTORE: u32 = 23; // Where the player must bring treasure to
pub const LOCATION_ID_TREETOP: u32 = 28; // The tree in the garden
pub const LOCATION_ID_WITCH_0: u32 = 107; // Location of teleporter connected to Experiment Area
pub const LOCATION_ID_WITCH_1: u32 = 128; // Location of teleporter connected to Chasm

pub const LOCATION_ID_SAFE_INITIAL: u32 = 34; // Safe location before pirates arrive
pub const LOCATION_ID_SAFE_PIRATES: u32 = 50; // Safe location after pirates arrive
pub const LOCATION_ID_WAKE_INITIAL: u32 = 9; // Wake location before pirates arrive
pub const LOCATION_ID_WAKE_PIRATES: u32 = 79; // Wake location after pirates arrive

pub const ITEM_INDEX_START: u32 = 1000; // ID numbers before this index are used for locations, everything from here on for items

// ID numbers of specific items
pub const ITEM_ID_ACORN: u32 = 1082;
pub const ITEM_ID_ALIEN: u32 = 1000;
pub const ITEM_ID_AQUA: u32 = 1084;
pub const ITEM_ID_BOOTS: u32 = 1006;
pub const ITEM_ID_BOULDER: u32 = 1008;
pub const ITEM_ID_BREAD: u32 = 1010;
pub const ITEM_ID_BROOCH: u32 = 1087;
pub const ITEM_ID_BUCCANEER: u32 = 1014;
pub const ITEM_ID_BUTTON: u32 = 1013;
pub const ITEM_ID_CARTRIDGE: u32 = 1017;
pub const ITEM_ID_CASSETTE: u32 = 1120;
pub const ITEM_ID_CAULDRON: u32 = 1018;
pub const ITEM_ID_CD: u32 = 1119;
pub const ITEM_ID_CHART: u32 = 1019;
pub const ITEM_ID_COIN: u32 = 1089;
pub const ITEM_ID_CONSOLE_BROKEN: u32 = 1021;
pub const ITEM_ID_CONSOLE_FIXED: u32 = 1057;
pub const ITEM_ID_CORSAIR: u32 = 1022;
pub const ITEM_ID_DOGS: u32 = 1026;
pub const ITEM_ID_DUST: u32 = 1092;
pub const ITEM_ID_DRAGON: u32 = 1027;
pub const ITEM_ID_ELIXIR: u32 = 1028;
pub const ITEM_ID_ENVELOPE: u32 = 1029;
pub const ITEM_ID_FAIRY: u32 = 1030;
pub const ITEM_ID_GLINT: u32 = 1033;
pub const ITEM_ID_GUNSLINGER: u32 = 1037;
pub const ITEM_ID_JUMPER: u32 = 1040;
pub const ITEM_ID_KEY: u32 = 1041;
pub const ITEM_ID_KOHLRABI: u32 = 1042;
pub const ITEM_ID_LAMP: u32 = 1043;
pub const ITEM_ID_LENS: u32 = 1044;
pub const ITEM_ID_LION: u32 = 1045;
pub const ITEM_ID_MAGAZINE: u32 = 1047;
pub const ITEM_ID_MATCHES: u32 = 1048;
pub const ITEM_ID_MILK: u32 = 1049;
pub const ITEM_ID_MIRROR: u32 = 1050;
pub const ITEM_ID_NEEDLES: u32 = 1052;
pub const ITEM_ID_NET: u32 = 1053;
pub const ITEM_ID_NUGGET: u32 = 1096;
pub const ITEM_ID_PENDANT: u32 = 1097;
pub const ITEM_ID_PLAYER: u32 = 1103;
pub const ITEM_ID_POTION: u32 = 1059;
pub const ITEM_ID_RADISHES: u32 = 1060;
pub const ITEM_ID_ROBOT: u32 = 1061;
pub const ITEM_ID_ROD: u32 = 1099;
pub const ITEM_ID_SHIP: u32 = 1062;
pub const ITEM_ID_SKELETON: u32 = 1065;
pub const ITEM_ID_STEW: u32 = 1115;
pub const ITEM_ID_TOAST: u32 = 1069;
pub const ITEM_ID_TOOTH: u32 = 1070;
pub const ITEM_ID_TRANSMITTER: u32 = 1071;
pub const ITEM_ID_TROLL: u32 = 1073;
pub const ITEM_ID_WATER: u32 = 1076;
pub const ITEM_ID_WHISTLE: u32 = 1077;
pub const ITEM_ID_WIRE: u32 = 1078;
pub const ITEM_ID_WIZARD: u32 = 1079;
pub const ITEM_ID_WOLF: u32 = 1080;
pub const ITEM_ID_YARN: u32 = 1081;
