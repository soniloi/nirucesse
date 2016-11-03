// Validation for various collections
pub const EXPECTED_ITEMS: u32 = 124;
pub const EXPECTED_LOCATIONS: u32 = 134;
pub const EXPECTED_STRINGS_RESPONSES: u32 = 193;
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

// Capacity
pub const INVENTORY_CAPACITY: u32 = 16;

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
pub const LOCATION_ID_HOT: u32 = 87; // Hot room
pub const LOCATION_ID_INVENTORY: u32 = 1; // Dummy value
pub const LOCATION_ID_NURSERY: u32 = 3; // Default location of items before they appear after solving puzzles etc.
pub const LOCATION_ID_OBSERVATORY: u32 = 126; // Where the alien hangs out
pub const LOCATION_ID_REFLECTION: u32 = 120; // The mirror room
pub const LOCATION_ID_SENSOR: u32 = 52; // Sensor room off the Control Hub
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
pub const ITEM_ID_BEAN: u32 = 1003;
pub const ITEM_ID_BEANSTALK: u32 = 1004;
pub const ITEM_ID_BLOSSOM: u32 = 1085;
pub const ITEM_ID_BODIES: u32 = 1117;
pub const ITEM_ID_BOOK: u32 = 1105;
pub const ITEM_ID_BOOTS: u32 = 1006;
pub const ITEM_ID_BOULDER: u32 = 1008;
pub const ITEM_ID_BREAD: u32 = 1010;
pub const ITEM_ID_BROOCH: u32 = 1087;
pub const ITEM_ID_BUCCANEER: u32 = 1014;
pub const ITEM_ID_BUILDING: u32 = 1012;
pub const ITEM_ID_BUTTON: u32 = 1013;
pub const ITEM_ID_CABLE: u32 = 1015;
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
pub const ITEM_ID_MACHINE: u32 = 1046;
pub const ITEM_ID_MAGAZINE: u32 = 1047;
pub const ITEM_ID_MATCHES: u32 = 1048;
pub const ITEM_ID_MEDALLION: u32 = 1094;
pub const ITEM_ID_MILK: u32 = 1049;
pub const ITEM_ID_MIRROR: u32 = 1050;
pub const ITEM_ID_NEEDLES: u32 = 1052;
pub const ITEM_ID_NET: u32 = 1053;
pub const ITEM_ID_NUGGET: u32 = 1096;
pub const ITEM_ID_PENDANT: u32 = 1097;
pub const ITEM_ID_PLANT: u32 = 1058;
pub const ITEM_ID_PLAYER: u32 = 1103;
pub const ITEM_ID_POTION: u32 = 1059;
pub const ITEM_ID_RADISHES: u32 = 1060;
pub const ITEM_ID_ROBOT: u32 = 1061;
pub const ITEM_ID_ROD: u32 = 1099;
pub const ITEM_ID_SHIP: u32 = 1062;
pub const ITEM_ID_SHUTTLE: u32 = 1063;
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

pub const STR_ID_ALIEN_NO_USE: u32 = 1;
pub const STR_ID_ALREADY_DONE: u32 = 2;
pub const STR_ID_ROBOT_MOUSE: u32 = 6;
pub const STR_ID_REINCARNATE_ASK: u32 = 8;
pub const STR_ID_SURE_ASK: u32 = 9;
pub const STR_ID_BOULDER_HIT_WEAK: u32 = 11;
pub const STR_ID_BURN_BREAD: u32 = 12;
pub const STR_ID_CABBAGE: u32 = 13;
pub const STR_ID_COOK_CABBAGE: u32 = 14;
pub const STR_ID_NO_SEE_DARKNESS: u32 = 15;
pub const STR_ID_NO_SEE_HAZE: u32 = 16;
pub const STR_ID_CAULDRON_FULL: u32 = 17;
pub const STR_ID_CONTAINER_FULL: u32 = 20;
pub const STR_ID_ALREADY_CONTAINED: u32 = 21;
pub const STR_ID_NOT_LIQUID_CONTAINER: u32 = 22;
pub const STR_ID_NOT_SOLID_CONTAINER: u32 = 23;
pub const STR_ID_NOT_CONTAINER: u32 = 24;
pub const STR_ID_CONTAINER_INTO_SELF: u32 = 25;
pub const STR_ID_IT_IS: u32 = 26;
pub const STR_ID_DEAD: u32 = 27;
pub const STR_ID_REINCARNATE_DO: u32 = 28;
pub const STR_ID_DOT: u32 = 29;
pub const STR_ID_DRINK_LIQUID: u32 = 30;
pub const STR_ID_DRINK_AQUA: u32 = 31;
pub const STR_ID_DRINK_ELIXIR: u32 = 32;
pub const STR_ID_DRINK_POTION: u32 = 33;
pub const STR_ID_DRINK_STEW: u32 = 34;
pub const STR_ID_DRINK_WATER: u32 = 35;
pub const STR_ID_DROP_NO_FLOOR: u32 = 36;
pub const STR_ID_DROP_GOOD: u32 = 37;
pub const STR_ID_ALREADY_EMPTY: u32 = 40;
pub const STR_ID_EMPTY_CARRY: u32 = 41;
pub const STR_ID_EMPTY_LIQUID: u32 = 42;
pub const STR_ID_EMPTY_SET: u32 = 43;
pub const STR_ID_RUB_LAMP: u32 = 47;
pub const STR_ID_DISAMBIGUATE_GO: u32 = 49;
pub const STR_ID_WELCOME: u32 = 50;
pub const STR_ID_HELLO_BEACON: u32 = 51;
pub const STR_ID_HELLO_CHART: u32 = 52;
pub const STR_ID_HELLO_LENS: u32 = 53;
pub const STR_ID_HINT_FOUND: u32 = 54;
pub const STR_ID_IGNORED: u32 = 55;
pub const STR_ID_AWAKEN_INITIAL: u32 = 57;
pub const STR_ID_INSERTED: u32 = 58;
pub const STR_ID_LION_CABBAGE: u32 = 60;
pub const STR_ID_LION_WHET: u32 = 61;
pub const STR_ID_DONE: u32 = 62;
pub const STR_ID_MACHINE_REJECT: u32 = 63;
pub const STR_ID_MACHINE_ASK: u32 = 64;
pub const STR_ID_MACHINE_NO_KNOW_WHAT: u32 = 65;
pub const STR_ID_NO_AIR: u32 = 66;
pub const STR_ID_NO_REACH_CEILING: u32 = 67;
pub const STR_ID_DOWN_KILL: u32 = 68;
pub const STR_ID_OPEN_WATER: u32 = 69;
pub const STR_ID_NO_IN_OUT: u32 = 70;
pub const STR_ID_NO_REMEMBER: u32 = 71;
pub const STR_ID_CANNOT_GO: u32 = 72;
pub const STR_ID_SUFFOCATE: u32 = 73;
pub const STR_ID_NO_HAVE_INVENTORY: u32 = 74;
pub const STR_ID_NO_HERE_COOK: u32 = 76;
pub const STR_ID_NO_EQUIPMENT: u32 = 77;
pub const STR_ID_NOWHERE_EXCHANGE: u32 = 78;
pub const STR_ID_NOT_FEEDABLE: u32 = 79;
pub const STR_ID_NO_FISH: u32 = 80;
pub const STR_ID_NO_FIT: u32 = 81;
pub const STR_ID_CANNOT_INSERT_WEARABLE: u32 = 82;
pub const STR_ID_LION_SITS: u32 = 83;
pub const STR_ID_DEATH_NO_GRAVITY: u32 = 85;
pub const STR_ID_NOTHING_HAPPENS: u32 = 86;
pub const STR_ID_NOT_INTERESTED: u32 = 88;
pub const STR_ID_NOTHING_INTERESTING: u32 = 89;
pub const STR_ID_BREAK_NECK: u32 = 91;
pub const STR_ID_NO_CARRY_BURN: u32 = 92;
pub const STR_ID_DRINK_NON_LIQUID: u32 = 93;
pub const STR_ID_NO_KNOW_HOW: u32 = 94;
pub const STR_ID_NO_MUSIC: u32 = 95;
pub const STR_ID_PLAY_CD: u32 = 96;
pub const STR_ID_PLAY_CASSETTE: u32 = 97;
pub const STR_ID_NO_KNOW_WHO_WHAT: u32 = 98;
pub const STR_ID_POUR_NONLIQUID: u32 = 99;
pub const STR_ID_NO_SEE_HERE: u32 = 100;
pub const STR_ID_NO_KNOW_APPLY: u32 = 101;
pub const STR_ID_NO_UNDERSTAND_INSTRUCTION: u32 = 103;
pub const STR_ID_NO_UNDERSTAND_SELECTION: u32 = 104;
pub const STR_ID_NOT_VALUABLE: u32 = 105;
pub const STR_ID_UNWISE: u32 = 106;
pub const STR_ID_NO_WRITING: u32 = 107;
pub const STR_ID_BLOCKED: u32 = 108;
pub const STR_ID_OK: u32 = 110;
pub const STR_ID_RUB_PENDANT: u32 = 111;
pub const STR_ID_PHILISTINE: u32 = 112;
pub const STR_ID_CORSAIR_SNEAK_ROB: u32 = 113;
pub const STR_ID_CORSAIR_SPEAK: u32 = 114;
pub const STR_ID_PIRATE_EMPTY: u32 = 115;
pub const STR_ID_BUCCANEER_SNEAK_ROB: u32 = 116;
pub const STR_ID_CORSAIR_SNEAK_PAST: u32 = 117;
pub const STR_ID_BUCCANEER_SNEAK_PAST: u32 = 118;
pub const STR_ID_CORSAIR_LISTENING: u32 = 119;
pub const STR_ID_BUCCANEER_WATCHING: u32 = 120;
pub const STR_ID_PLAY_WHISTLE: u32 = 121;
pub const STR_ID_HOLLOW: u32 = 122;
pub const STR_ID_SEE_INVISIBLE: u32 = 124;
pub const STR_ID_SEE_NORMAL: u32 = 125;
pub const STR_ID_SEE_NOTHING: u32 = 126;
pub const STR_ID_SEE_STRONG: u32 = 127;
pub const STR_ID_SCORE_DIED: u32 = 128;
pub const STR_ID_SCORE_DEATHS: u32 = 129;
pub const STR_ID_SCORE_HINTS: u32 = 130;
pub const STR_ID_SCORE_INSTRUCTIONS: u32 = 131;
pub const STR_ID_SCORE_CURRENT: u32 = 132;
pub const STR_ID_SCORE_POINTS: u32 = 133;
pub const STR_ID_SCORE_FINAL: u32 = 134;
pub const STR_ID_BREAK_FAR: u32 = 135;
pub const STR_ID_BREAK_NEAR: u32 = 136;
pub const STR_ID_BAD_LUCK: u32 = 137;
pub const STR_ID_SH_MAGIC: u32 = 139;
pub const STR_ID_SLEEP: u32 = 140;
pub const STR_ID_NO_SLEEP: u32 = 141;
pub const STR_ID_SNOMP_KILL: u32 = 142;
pub const STR_ID_ALREADY_HAVE: u32 = 145;
pub const STR_ID_CANNOT_TAKE: u32 = 146;
pub const STR_ID_ITEM_HEAVY: u32 = 147;
pub const STR_ID_TAKEN: u32 = 148;
pub const STR_ID_THE_START: u32 = 149;
pub const STR_ID_THROW: u32 = 151;
pub const STR_ID_BURN_TOAST: u32 = 152;
pub const STR_ID_NO_GRAVITY: u32 = 153;
pub const STR_ID_TROLL_FED: u32 = 154;
pub const STR_ID_WORN: u32 = 156;
pub const STR_ID_ALREADY_REPAIRED: u32 = 157;
pub const STR_ID_NO_COMPONENT: u32 = 158;
pub const STR_ID_WHAT_FEED_ACC: u32 = 159;
pub const STR_ID_WHAT_FEED_DAT: u32 = 160;
pub const STR_ID_WHAT_INSERT: u32 = 161;
pub const STR_ID_ARG_GET: u32 = 162;
pub const STR_ID_WHAT_PLAY: u32 = 163;
pub const STR_ID_WITCHED: u32 = 165;
pub const STR_ID_WIZARDED: u32 = 166;
pub const STR_ID_WIZARD_INVISIBLE: u32 = 167;
pub const STR_ID_WHAT_GIVE: u32 = 168;
pub const STR_ID_READS: u32 = 169;
pub const STR_ID_SAY: u32 = 170;
pub const STR_ID_HELLO: u32 = 171;
pub const STR_ID_GLINT_HEAVY: u32 = 172;
pub const STR_ID_PIRATE_HEAVY: u32 = 173;
pub const STR_ID_WHAT_POUR: u32 = 174;
pub const STR_ID_POUR_LIQUID_DEFAULT: u32 = 175;
pub const STR_ID_POUR_POTION_BEAN: u32 = 176;
pub const STR_ID_POUR_POTION_PLANT: u32 = 177;
pub const STR_ID_NODE: u32 = 178;
pub const STR_ID_NO_WANT_TAKE: u32 = 179;
pub const STR_ID_GRABBED: u32 = 180;
pub const STR_ID_DISAMBIGUATE_CLIMB: u32 = 181;
pub const STR_ID_ARG_EXTRA: u32 = 182;
pub const STR_ID_DISAMBIGUATE_WATER: u32 = 183;
pub const STR_ID_NO_TETHER: u32 = 184;
pub const STR_ID_WHAT_TETHER: u32 = 185;
pub const STR_ID_EXCHANGE_GOOD: u32 = 186;
pub const STR_ID_BUY_FARM: u32 = 187;
pub const STR_ID_MACHINE_NO_KNOW_CREATE: u32 = 188;
pub const STR_ID_MACHINE_ALREADY_CREATE: u32 = 189;
pub const STR_ID_MACHINE_DISPENSE: u32 = 190;
pub const STR_ID_DISAMBIGUATE_LOOK: u32 = 191;
pub const STR_ID_NO: u32 = 192;

pub const PUZZLE_ID_ACORN: u32 = 0;
pub const PUZZLE_ID_TRANSMITTER: u32 = 1;
pub const PUZZLE_ID_CHART: u32 = 2;
pub const PUZZLE_ID_LENS: u32 = 3;
pub const PUZZLE_ID_BEANSTALK: u32 = 4;
pub const PUZZLE_ID_BOULDER: u32 = 5;
pub const PUZZLE_ID_CONSOLE: u32 = 6;
pub const PUZZLE_ID_DISTRESS: u32 = 7;
pub const PUZZLE_ID_DRAGON: u32 = 8;
pub const PUZZLE_ID_FAIRY: u32 = 9;
pub const PUZZLE_ID_GUNSLINGER: u32 = 10;
pub const PUZZLE_ID_JUMPER: u32 = 11;
pub const PUZZLE_ID_LION: u32 = 12;
pub const PUZZLE_ID_WOLF: u32 = 13;
pub const PUZZLE_ID_GLINT: u32 = 14;
pub const PUZZLE_ID_CORSAIR: u32 = 15;
pub const PUZZLE_ID_BUCCANEER: u32 = 16;
pub const PUZZLE_ID_ELIXIR: u32 = 17;
pub const PUZZLE_ID_ROBOT: u32 = 18;
pub const PUZZLE_ID_SKELETON: u32 = 19;
pub const PUZZLE_ID_TETHER: u32 = 20;
pub const PUZZLE_ID_AIRLOCK: u32 = 21;
pub const PUZZLE_ID_TROLL: u32 = 22;
pub const PUZZLE_ID_WIZARD: u32 = 23;

// Attribute codes for items
pub const CTRL_ITEM_CONTAINER: u32 = 0x1;  // Whether an item may contain other items
pub const CTRL_ITEM_MOBILE: u32 = 0x2; // Whether an item is fixed or mobile (carryable)
pub const CTRL_ITEM_OBSTRUCTION: u32 = 0x4; // Whether an item is an obstruction
pub const CTRL_ITEM_SWITCHABLE: u32 = 0x8; // Whether an item can be lit/quenched
pub const CTRL_ITEM_GIVES_LIGHT: u32 = 0x10; // Whether an item emits light
pub const CTRL_ITEM_GIVES_AIR: u32 = 0x20; // Whether an item enables player to breathe
pub const CTRL_ITEM_GIVES_GRAVITY: u32 = 0x40; // Whether an item holds the player down
pub const CTRL_ITEM_GIVES_NOSNOMP: u32 = 0x80; // Whether an item protects the player from snomps
pub const CTRL_ITEM_CONTAINER_LIQUID: u32 = 0x100; // Whether the container may contain liquids rather than solids
pub const CTRL_ITEM_FRAGILE: u32 = 0x200; // Whether an item would survive throwing, dropping from heights, etc
pub const CTRL_ITEM_WEARABLE: u32 = 0x400; // Whether an item is to be worn by the player rather than carried
pub const CTRL_ITEM_LIQUID: u32 = 0x800; // Whether item is a liquid, i.e. needs a special container to carry it
pub const CTRL_ITEM_ESSENTIAL: u32 = 0x1000; // Whether an item is essential to basic gameplay
pub const CTRL_ITEM_EDIBLE: u32 = 0x2000; // Whether an item is any sort of food or drink
pub const CTRL_ITEM_GIVES_INVISIBILITY: u32 = 0x4000; // Whether wearing or carrying an item makes the player invisible
pub const CTRL_ITEM_TREASURE: u32 = 0x8000; // Whether an item is a treasure
pub const CTRL_ITEM_FACTORY: u32 = 0x10000; // Whether an item can be made by the machine
pub const CTRL_ITEM_SILENT: u32 = 0x20000; // Whether an item should be shown in location descriptions
pub const CTRL_ITEM_GIVES_LAND: u32 = 0x40000; // Whether an item acts as 'land' i.e. a boat or whatever
pub const CTRL_ITEM_RECIPIENT: u32 = 0x80000; // Whether an item may be a recipient (i.e. of gifts or food)

// Attribute codes for locations
pub const CTRL_LOC_HAS_LIGHT: u32 = 0x01; // Whether the location has ambient lighting
pub const CTRL_LOC_HAS_AIR: u32 = 0x2; // Whether there is air at the location
pub const CTRL_LOC_HAS_GRAVITY: u32 = 0x4; // Whether there is gravity at the location
pub const CTRL_LOC_HAS_NOSNOMP: u32 = 0x8; // Whether there is absence of snomps at the location
pub const CTRL_LOC_NEEDSNO_LIGHT: u32 = 0x10; // Whether the location requires no portable lighting in order for the player to be able to see clearly
pub const CTRL_LOC_NEEDSNO_GRAVITY: u32 = 0x40; // Whether the location requires that there be no gravity
pub const CTRL_LOC_HAS_CEILING: u32 = 0x100; // Whether there is a ceiling to this location, or something above it
pub const CTRL_LOC_HAS_FLOOR: u32 = 0x200; // Whether there is a floor at this location
pub const CTRL_LOC_HAS_LAND: u32 = 0x400; // Whether the location has land, as opposed to open water
