use libc::{uint16_t};

pub const INPUT_PROP_POINTER:        uint16_t = 0x00; /* needs a pointer */
pub const INPUT_PROP_DIRECT:         uint16_t = 0x01; /* direct input devices */
pub const INPUT_PROP_BUTTONPAD:      uint16_t = 0x02; /* has button: uint16_t = s under pad */
pub const INPUT_PROP_SEMI_MT:        uint16_t = 0x03; /* touch rectangle only */
pub const INPUT_PROP_TOPBUTTONPAD:   uint16_t = 0x04; /* softbuttons at top of pad */
pub const INPUT_PROP_POINTING_STICK: uint16_t = 0x05; /* is a pointing stick */
pub const INPUT_PROP_ACCELEROMETER:  uint16_t = 0x06; /* has accelerometer */

pub const INPUT_PROP_MAX: uint16_t = 0x1f;
pub const INPUT_PROP_CNT: uint16_t = INPUT_PROP_MAX + 1;

/*
 * Event types
 */

pub const EV_SYN:       uint16_t = 0x00;
pub const EV_KEY:       uint16_t = 0x01;
pub const EV_REL:       uint16_t = 0x02;
pub const EV_ABS:       uint16_t = 0x03;
pub const EV_MSC:       uint16_t = 0x04;
pub const EV_SW:        uint16_t = 0x05;
pub const EV_LED:       uint16_t = 0x11;
pub const EV_SND:       uint16_t = 0x12;
pub const EV_REP:       uint16_t = 0x14;
pub const EV_FF:        uint16_t = 0x15;
pub const EV_PWR:       uint16_t = 0x16;
pub const EV_FF_STATUS: uint16_t = 0x17;
pub const EV_MAX:       uint16_t = 0x1f;
pub const EV_CNT:       uint16_t = EV_MAX + 1;

/*
 * Synchronization events.
 */

pub const SYN_REPORT:    uint16_t = 0;
pub const SYN_CONFIG:    uint16_t = 1;
pub const SYN_MT_REPORT: uint16_t = 2;
pub const SYN_DROPPED:   uint16_t = 3;
pub const SYN_MAX:       uint16_t = 0xf;
pub const SYN_CNT:       uint16_t = SYN_MAX + 1;

/*
 * Keys and buttons
 *
 * Most of the keys/buttons are modeled after USB HUT 1.12 (see
 * http://www.usb.org/developers/hidpage).
 * Abbreviations in the comments:
 * AC - Application Control
 * AL - Application Launch Button
 * SC - System Control
 */

pub const KEY_RESERVED:   uint16_t = 0;
pub const KEY_ESC:        uint16_t = 1;
pub const KEY_1:          uint16_t = 2;
pub const KEY_2:          uint16_t = 3;
pub const KEY_3:          uint16_t = 4;
pub const KEY_4:          uint16_t = 5;
pub const KEY_5:          uint16_t = 6;
pub const KEY_6:          uint16_t = 7;
pub const KEY_7:          uint16_t = 8;
pub const KEY_8:          uint16_t = 9;
pub const KEY_9:          uint16_t = 10;
pub const KEY_10:         uint16_t = 11;
pub const KEY_MINUS:      uint16_t = 12;
pub const KEY_EQUAL:      uint16_t = 13;
pub const KEY_BACKSPACE:  uint16_t = 14;
pub const KEY_TAB:        uint16_t = 15;
pub const KEY_Q:          uint16_t = 16;
pub const KEY_W:          uint16_t = 17;
pub const KEY_E:          uint16_t = 18;
pub const KEY_R:          uint16_t = 19;
pub const KEY_T:          uint16_t = 20;
pub const KEY_Y:          uint16_t = 21;
pub const KEY_U:          uint16_t = 22;
pub const KEY_I:          uint16_t = 23;
pub const KEY_O:          uint16_t = 24;
pub const KEY_P:          uint16_t = 25;
pub const KEY_LEFTBRACE:  uint16_t = 26;
pub const KEY_RIGHTBRACE: uint16_t = 27;
pub const KEY_ENTER:      uint16_t = 28;
pub const KEY_LEFTCTRL:   uint16_t = 29;
pub const KEY_A:          uint16_t = 30;
pub const KEY_S:          uint16_t = 31;
pub const KEY_D:          uint16_t = 32;
pub const KEY_F:          uint16_t = 33;
pub const KEY_G:          uint16_t = 34;
pub const KEY_H:          uint16_t = 35;
pub const KEY_J:          uint16_t = 36;
pub const KEY_K:          uint16_t = 37;
pub const KEY_L:          uint16_t = 38;
pub const KEY_SEMICOLON:  uint16_t = 39;
pub const KEY_APOSTROPHE: uint16_t = 40;
pub const KEY_GRAVE:      uint16_t = 41;
pub const KEY_LEFTSHIFT:  uint16_t = 42;
pub const KEY_BACKSLASH:  uint16_t = 43;
pub const KEY_Z:          uint16_t = 44;
pub const KEY_X:          uint16_t = 45;
pub const KEY_C:          uint16_t = 46;
pub const KEY_V:          uint16_t = 47;
pub const KEY_B:          uint16_t = 48;
pub const KEY_N:          uint16_t = 49;
pub const KEY_M:          uint16_t = 50;
pub const KEY_COMMA:      uint16_t = 51;
pub const KEY_DOT:        uint16_t = 52;
pub const KEY_SLASH:      uint16_t = 53;
pub const KEY_RIGHTSHIFT: uint16_t = 54;
pub const KEY_KPASTERISK: uint16_t = 55;
pub const KEY_LEFTALT:    uint16_t = 56;
pub const KEY_SPACE:      uint16_t = 57;
pub const KEY_CAPSLOCK:   uint16_t = 58;
pub const KEY_F1:         uint16_t = 59;
pub const KEY_F2:         uint16_t = 60;
pub const KEY_F3:         uint16_t = 61;
pub const KEY_F4:         uint16_t = 62;
pub const KEY_F5:         uint16_t = 63;
pub const KEY_F6:         uint16_t = 64;
pub const KEY_F7:         uint16_t = 65;
pub const KEY_F8:         uint16_t = 66;
pub const KEY_F9:         uint16_t = 67;
pub const KEY_F10:        uint16_t = 68;
pub const KEY_NUMLOCK:    uint16_t = 69;
pub const KEY_SCROLLLOCK: uint16_t = 70;
pub const KEY_KP7:        uint16_t = 71;
pub const KEY_KP8:        uint16_t = 72;
pub const KEY_KP9:        uint16_t = 73;
pub const KEY_KPMINUS:    uint16_t = 74;
pub const KEY_KP4:        uint16_t = 75;
pub const KEY_KP5:        uint16_t = 76;
pub const KEY_KP6:        uint16_t = 77;
pub const KEY_KPPLUS:     uint16_t = 78;
pub const KEY_KP1:        uint16_t = 79;
pub const KEY_KP2:        uint16_t = 80;
pub const KEY_KP3:        uint16_t = 81;
pub const KEY_KP0:        uint16_t = 82;
pub const KEY_KPDOT:      uint16_t = 83;

pub const KEY_ZENKAKUHANKAKU:   uint16_t = 85;
pub const KEY_102ND:            uint16_t = 86;
pub const KEY_F11:              uint16_t = 87;
pub const KEY_F12:              uint16_t = 88;
pub const KEY_RO:               uint16_t = 89;
pub const KEY_KATAKANA:         uint16_t = 90;
pub const KEY_HIRAGANA:         uint16_t = 91;
pub const KEY_HENKAN:           uint16_t = 92;
pub const KEY_KATAKANAHIRAGANA: uint16_t = 93;
pub const KEY_MUHENKAN:         uint16_t = 94;
pub const KEY_KPJPCOMMA:        uint16_t = 95;
pub const KEY_KPENTER:          uint16_t = 96;
pub const KEY_RIGHTCTRL:        uint16_t = 97;
pub const KEY_KPSLASH:          uint16_t = 98;
pub const KEY_SYSRQ:            uint16_t = 99;
pub const KEY_RIGHTALT:         uint16_t = 100;
pub const KEY_LINEFEED:         uint16_t = 101;
pub const KEY_HOME:             uint16_t = 102;
pub const KEY_UP:               uint16_t = 103;
pub const KEY_PAGEUP:           uint16_t = 104;
pub const KEY_LEFT:             uint16_t = 105;
pub const KEY_RIGHT:            uint16_t = 106;
pub const KEY_END:              uint16_t = 107;
pub const KEY_DOWN:             uint16_t = 108;
pub const KEY_PAGEDOWN:         uint16_t = 109;
pub const KEY_INSERT:           uint16_t = 110;
pub const KEY_DELETE:           uint16_t = 111;
pub const KEY_MACRO:            uint16_t = 112;
pub const KEY_MUTE:             uint16_t = 113;
pub const KEY_VOLUMEDOWN:       uint16_t = 114;
pub const KEY_VOLUMEUP:         uint16_t = 115;
pub const KEY_POWER:            uint16_t = 116; /* SC System Power Down */
pub const KEY_KPEQUAL:          uint16_t = 117;
pub const KEY_KPPLUSMINUS:      uint16_t = 118;
pub const KEY_PAUSE:            uint16_t = 119;
pub const KEY_SCALE:            uint16_t = 120; /* AL Compiz Scale : uint16_t = Expose */

pub const KEY_KPCOMMA:   uint16_t = 121;
pub const KEY_HANGEUL:   uint16_t = 122;
pub const KEY_HANGUEL:   uint16_t = KEY_HANGEUL;
pub const KEY_HANJA:     uint16_t = 123;
pub const KEY_YEN:       uint16_t = 124;
pub const KEY_LEFTMETA:  uint16_t = 125;
pub const KEY_RIGHTMETA: uint16_t = 126;
pub const KEY_COMPOSE:   uint16_t = 127;

pub const KEY_STOP:           uint16_t = 128; /* AC Stop */
pub const KEY_AGAIN:          uint16_t = 129;
pub const KEY_PROPS:          uint16_t = 130; /* AC Properties */
pub const KEY_UNDO:           uint16_t = 131; /* AC Undo */
pub const KEY_FRONT:          uint16_t = 132;
pub const KEY_COPY:           uint16_t = 133; /* AC Copy */
pub const KEY_OPEN:           uint16_t = 134; /* AC Open */
pub const KEY_PASTE:          uint16_t = 135; /* AC Paste */
pub const KEY_FIND:           uint16_t = 136; /* AC Search */
pub const KEY_CUT:            uint16_t = 137; /* AC Cut */
pub const KEY_HELP:           uint16_t = 138; /* AL Integrated Help Center */
pub const KEY_MENU:           uint16_t = 139; /* Menu : uint16_t = show menu */
pub const KEY_CALC:           uint16_t = 140; /* AL Calculator */
pub const KEY_SETUP:          uint16_t = 141;
pub const KEY_SLEEP:          uint16_t = 142; /* SC System Sleep */
pub const KEY_WAKEUP:         uint16_t = 143; /* System Wake Up */
pub const KEY_FILE:           uint16_t = 144; /* AL Local Machine Browser */
pub const KEY_SENDFILE:       uint16_t = 145;
pub const KEY_DELETEFILE:     uint16_t = 146;
pub const KEY_XFER:           uint16_t = 147;
pub const KEY_PROG1:          uint16_t = 148;
pub const KEY_PROG2:          uint16_t = 149;
pub const KEY_WWW:            uint16_t = 150; /* AL Internet Browser */
pub const KEY_MSDOS:          uint16_t = 151;
pub const KEY_COFFEE:         uint16_t = 152; /* AL Terminal Lock/Screensaver */
pub const KEY_SCREENLOCK:     uint16_t = KEY_COFFEE;
pub const KEY_ROTATE_DISPLAY: uint16_t = 153; /* Display orientation for e.g. tablets */
pub const KEY_DIRECTION:      uint16_t = KEY_ROTATE_DISPLAY;
pub const KEY_CYCLEWINDOWS:   uint16_t = 154;
pub const KEY_MAIL:           uint16_t = 155;
pub const KEY_BOOKMARKS:      uint16_t = 156; /* AC Bookmarks */
pub const KEY_COMPUTER:       uint16_t = 157;
pub const KEY_BACK:           uint16_t = 158; /* AC Back */
pub const KEY_FORWARD:        uint16_t = 159; /* AC Forward */
pub const KEY_CLOSECD:        uint16_t = 160;
pub const KEY_EJECTCD:        uint16_t = 161;
pub const KEY_EJECTCLOSECD:   uint16_t = 162;
pub const KEY_NEXTSONG:       uint16_t = 163;
pub const KEY_PLAYPAUSE:      uint16_t = 164;
pub const KEY_PREVIOUSSONG:   uint16_t = 165;
pub const KEY_STOPCD:         uint16_t = 166;
pub const KEY_RECORD:         uint16_t = 167;
pub const KEY_REWIND:         uint16_t = 168;
pub const KEY_PHONE:          uint16_t = 169; /* Media Select Telephone */
pub const KEY_ISO:            uint16_t = 170;
pub const KEY_CONFIG:         uint16_t = 171; /* AL Consumer Control Configuration */
pub const KEY_HOMEPAGE:       uint16_t = 172; /* AC Home */
pub const KEY_REFRESH:        uint16_t = 173; /* AC Refresh */
pub const KEY_EXIT:           uint16_t = 174; /* AC Exit */
pub const KEY_MOVE:           uint16_t = 175;
pub const KEY_EDIT:           uint16_t = 176;
pub const KEY_SCROLLUP:       uint16_t = 177;
pub const KEY_SCROLLDOWN:     uint16_t = 178;
pub const KEY_KPLEFTPAREN:    uint16_t = 179;
pub const KEY_KPRIGHTPAREN:   uint16_t = 180;
pub const KEY_NEW:            uint16_t = 181; /* AC New */
pub const KEY_REDO:           uint16_t = 182; /* AC Redo/Repeat */

pub const KEY_F13: uint16_t = 183;
pub const KEY_F14: uint16_t = 184;
pub const KEY_F15: uint16_t = 185;
pub const KEY_F16: uint16_t = 186;
pub const KEY_F17: uint16_t = 187;
pub const KEY_F18: uint16_t = 188;
pub const KEY_F19: uint16_t = 189;
pub const KEY_F20: uint16_t = 190;
pub const KEY_F21: uint16_t = 191;
pub const KEY_F22: uint16_t = 192;
pub const KEY_F23: uint16_t = 193;
pub const KEY_F24: uint16_t = 194;

pub const KEY_PLAYCD:         uint16_t = 200;
pub const KEY_PAUSECD:        uint16_t = 201;
pub const KEY_PROG3:          uint16_t = 202;
pub const KEY_PROG4:          uint16_t = 203;
pub const KEY_DASHBOARD:      uint16_t = 204; /* AL Dashboard */
pub const KEY_SUSPEND:        uint16_t = 205;
pub const KEY_CLOSE:          uint16_t = 206; /* AC Close */
pub const KEY_PLAY:           uint16_t = 207;
pub const KEY_FASTFORWARD:    uint16_t = 208;
pub const KEY_BASSBOOST:      uint16_t = 209;
pub const KEY_PRINT:          uint16_t = 210; /* AC Print */
pub const KEY_HP:             uint16_t = 211;
pub const KEY_CAMERA:         uint16_t = 212;
pub const KEY_SOUND:          uint16_t = 213;
pub const KEY_QUESTION:       uint16_t = 214;
pub const KEY_EMAIL:          uint16_t = 215;
pub const KEY_CHAT:           uint16_t = 216;
pub const KEY_SEARCH:         uint16_t = 217;
pub const KEY_CONNECT:        uint16_t = 218;
pub const KEY_FINANCE:        uint16_t = 219; /* AL Checkbook/Finance */
pub const KEY_SPORT:          uint16_t = 220;
pub const KEY_SHOP:           uint16_t = 221;
pub const KEY_ALTERASE:       uint16_t = 222;
pub const KEY_CANCEL:         uint16_t = 223; /* AC Cancel */
pub const KEY_BRIGHTNESSDOWN: uint16_t = 224;
pub const KEY_BRIGHTNESSUP:   uint16_t = 225;
pub const KEY_MEDIA:          uint16_t = 226;

pub const KEY_SWITCHVIDEOMODE: uint16_t = 227; /* Cycle between available video outputs (Monitor/LCD/TV-out/etc) */
pub const KEY_KBDILLUMTOGGLE:  uint16_t = 228;
pub const KEY_KBDILLUMDOWN:    uint16_t = 229;
pub const KEY_KBDILLUMUP:      uint16_t = 230;

pub const KEY_SEND:        uint16_t = 231; /* AC Send */
pub const KEY_REPLY:       uint16_t = 232; /* AC Reply */
pub const KEY_FORWARDMAIL: uint16_t = 233; /* AC Forward Msg */
pub const KEY_SAVE:        uint16_t = 234; /* AC Save */
pub const KEY_DOCUMENTS:   uint16_t = 235;

pub const KEY_BATTERY: uint16_t = 236;

pub const KEY_BLUETOOTH: uint16_t = 237;
pub const KEY_WLAN:      uint16_t = 238;
pub const KEY_UWB:       uint16_t = 239;

pub const KEY_UNKNOWN: uint16_t = 240;

pub const KEY_VIDEO_NEXT:       uint16_t = 241; /* drive next video source */
pub const KEY_VIDEO_PREV:       uint16_t = 242; /* drive previous video source */
pub const KEY_BRIGHTNESS_CYCLE: uint16_t = 243; /* brightness up, after max is min */
pub const KEY_BRIGHTNESS_AUTO:  uint16_t = 244; /* Set Auto Brightness: manual brightness control is off, rely on ambient */
pub const KEY_BRIGHTNESS_ZERO:  uint16_t = KEY_BRIGHTNESS_AUTO;
pub const KEY_DISPLAY_OFF:      uint16_t = 245; /* display device to off state */

pub const KEY_WWAN:   uint16_t = 246; /* Wireless WAN : uint16_t = LTE, UMTS, GSM, etc. */
pub const KEY_WIMAX:  uint16_t = KEY_WWAN;
pub const KEY_RFKILL: uint16_t = 247; /* Key that controls all radios */

pub const KEY_MICMUTE: uint16_t = 248; /* Mute / unmute the microphone */

/* Code 255 is reserved for special needs of AT keyboard driver */

pub const BTN_MISC: uint16_t = 0x100;
pub const BTN_0:    uint16_t = 0x100;
pub const BTN_1:    uint16_t = 0x101;
pub const BTN_2:    uint16_t = 0x102;
pub const BTN_3:    uint16_t = 0x103;
pub const BTN_4:    uint16_t = 0x104;
pub const BTN_5:    uint16_t = 0x105;
pub const BTN_6:    uint16_t = 0x106;
pub const BTN_7:    uint16_t = 0x107;
pub const BTN_8:    uint16_t = 0x108;
pub const BTN_9:    uint16_t = 0x109;

pub const BTN_MOUSE:   uint16_t = 0x110;
pub const BTN_LEFT:    uint16_t = 0x110;
pub const BTN_RIGHT:   uint16_t = 0x111;
pub const BTN_MIDDLE:  uint16_t = 0x112;
pub const BTN_SIDE:    uint16_t = 0x113;
pub const BTN_EXTRA:   uint16_t = 0x114;
pub const BTN_FORWARD: uint16_t = 0x115;
pub const BTN_BACK:    uint16_t = 0x116;
pub const BTN_TASK:    uint16_t = 0x117;

pub const BTN_JOYSTICK: uint16_t = 0x120;
pub const BTN_TRIGGER:  uint16_t = 0x120;
pub const BTN_THUMB:    uint16_t = 0x121;
pub const BTN_THUMB2:   uint16_t = 0x122;
pub const BTN_TOP:      uint16_t = 0x123;
pub const BTN_TOP2:     uint16_t = 0x124;
pub const BTN_PINKIE:   uint16_t = 0x125;
pub const BTN_BASE:     uint16_t = 0x126;
pub const BTN_BASE2:    uint16_t = 0x127;
pub const BTN_BASE3:    uint16_t = 0x128;
pub const BTN_BASE4:    uint16_t = 0x129;
pub const BTN_BASE5:    uint16_t = 0x12a;
pub const BTN_BASE6:    uint16_t = 0x12b;
pub const BTN_DEAD:     uint16_t = 0x12f;

pub const BTN_GAMEPAD: uint16_t = 0x130;
pub const BTN_SOUTH:   uint16_t = 0x130;
pub const BTN_A:       uint16_t = BTN_SOUTH;
pub const BTN_EAST:    uint16_t = 0x131;
pub const BTN_B:       uint16_t = BTN_EAST;
pub const BTN_C:       uint16_t = 0x132;
pub const BTN_NORTH:   uint16_t = 0x133;
pub const BTN_X:       uint16_t = BTN_NORTH;
pub const BTN_WEST:    uint16_t = 0x134;
pub const BTN_Y:       uint16_t = BTN_WEST;
pub const BTN_Z:       uint16_t = 0x135;
pub const BTN_TL:      uint16_t = 0x136;
pub const BTN_TR:      uint16_t = 0x137;
pub const BTN_TL2:     uint16_t = 0x138;
pub const BTN_TR2:     uint16_t = 0x139;
pub const BTN_SELECT:  uint16_t = 0x13a;
pub const BTN_START:   uint16_t = 0x13b;
pub const BTN_MODE:    uint16_t = 0x13c;
pub const BTN_THUMBL:  uint16_t = 0x13d;
pub const BTN_THUMBR:  uint16_t = 0x13e;

pub const BTN_DIGI:           uint16_t = 0x140;
pub const BTN_TOOL_PEN:       uint16_t = 0x140;
pub const BTN_TOOL_RUBBER:    uint16_t = 0x141;
pub const BTN_TOOL_BRUSH:     uint16_t = 0x142;
pub const BTN_TOOL_PENCIL:    uint16_t = 0x143;
pub const BTN_TOOL_AIRBRUSH:  uint16_t = 0x144;
pub const BTN_TOOL_FINGER:    uint16_t = 0x145;
pub const BTN_TOOL_MOUSE:     uint16_t = 0x146;
pub const BTN_TOOL_LENS:      uint16_t = 0x147;
pub const BTN_TOOL_QUINTTAP:  uint16_t = 0x148; /* Five fingers on trackpad */
pub const BTN_TOUCH:          uint16_t = 0x14a;
pub const BTN_STYLUS:         uint16_t = 0x14b;
pub const BTN_STYLUS2:        uint16_t = 0x14c;
pub const BTN_TOOL_DOUBLETAP: uint16_t = 0x14d;
pub const BTN_TOOL_TRIPLETAP: uint16_t = 0x14e;
pub const BTN_TOOL_QUADTAP:   uint16_t = 0x14f; /* Four fingers on trackpad */

pub const BTN_WHEEL:     uint16_t = 0x150;
pub const BTN_GEAR_DOWN: uint16_t = 0x150;
pub const BTN_GEAR_UP:   uint16_t = 0x151;

pub const KEY_OK:                uint16_t = 0x160;
pub const KEY_SELECT:            uint16_t = 0x161;
pub const KEY_GOTO:              uint16_t = 0x162;
pub const KEY_CLEAR:             uint16_t = 0x163;
pub const KEY_POWER2:            uint16_t = 0x164;
pub const KEY_OPTION:            uint16_t = 0x165;
pub const KEY_INFO:              uint16_t = 0x166; /* AL OEM Features/Tips/Tutorial */
pub const KEY_TIME:              uint16_t = 0x167;
pub const KEY_VENDOR:            uint16_t = 0x168;
pub const KEY_ARCHIVE:           uint16_t = 0x169;
pub const KEY_PROGRAM:           uint16_t = 0x16a; /* Media Select Program Guide */
pub const KEY_CHANNEL:           uint16_t = 0x16b;
pub const KEY_FAVORITES:         uint16_t = 0x16c;
pub const KEY_EPG:               uint16_t = 0x16d;
pub const KEY_PVR:               uint16_t = 0x16e; /* Media Select Home */
pub const KEY_MHP:               uint16_t = 0x16f;
pub const KEY_LANGUAGE:          uint16_t = 0x170;
pub const KEY_TITLE:             uint16_t = 0x171;
pub const KEY_SUBTITLE:          uint16_t = 0x172;
pub const KEY_ANGLE:             uint16_t = 0x173;
pub const KEY_ZOOM:              uint16_t = 0x174;
pub const KEY_MODE:              uint16_t = 0x175;
pub const KEY_KEYBOARD:          uint16_t = 0x176;
pub const KEY_SCREEN:            uint16_t = 0x177;
pub const KEY_PC:                uint16_t = 0x178; /* Media Select Computer */
pub const KEY_TV:                uint16_t = 0x179; /* Media Select TV */
pub const KEY_TV2:               uint16_t = 0x17a; /* Media Select Cable */
pub const KEY_VCR:               uint16_t = 0x17b; /* Media Select VCR */
pub const KEY_VCR2:              uint16_t = 0x17c; /* VCR Plus */
pub const KEY_SAT:               uint16_t = 0x17d; /* Media Select Satellite */
pub const KEY_SAT2:              uint16_t = 0x17e;
pub const KEY_CD:                uint16_t = 0x17f; /* Media Select CD */
pub const KEY_TAPE:              uint16_t = 0x180; /* Media Select Tape */
pub const KEY_RADIO:             uint16_t = 0x181;
pub const KEY_TUNER:             uint16_t = 0x182; /* Media Select Tuner */
pub const KEY_PLAYER:            uint16_t = 0x183;
pub const KEY_TEXT:              uint16_t = 0x184;
pub const KEY_DVD:               uint16_t = 0x185; /* Media Select DVD */
pub const KEY_AUX:               uint16_t = 0x186;
pub const KEY_MP3:               uint16_t = 0x187;
pub const KEY_AUDIO:             uint16_t = 0x188; /* AL Audio Browser */
pub const KEY_VIDEO:             uint16_t = 0x189; /* AL Movie Browser */
pub const KEY_DIRECTORY:         uint16_t = 0x18a;
pub const KEY_LIST:              uint16_t = 0x18b;
pub const KEY_MEMO:              uint16_t = 0x18c; /* Media Select Messages */
pub const KEY_CALENDAR:          uint16_t = 0x18d;
pub const KEY_RED:               uint16_t = 0x18e;
pub const KEY_GREEN:             uint16_t = 0x18f;
pub const KEY_YELLOW:            uint16_t = 0x190;
pub const KEY_BLUE:              uint16_t = 0x191;
pub const KEY_CHANNELUP:         uint16_t = 0x192; /* Channel Increment */
pub const KEY_CHANNELDOWN:       uint16_t = 0x193; /* Channel Decrement */
pub const KEY_FIRST:             uint16_t = 0x194;
pub const KEY_LAST:              uint16_t = 0x195; /* Recall Last */
pub const KEY_AB:                uint16_t = 0x196;
pub const KEY_NEXT:              uint16_t = 0x197;
pub const KEY_RESTART:           uint16_t = 0x198;
pub const KEY_SLOW:              uint16_t = 0x199;
pub const KEY_SHUFFLE:           uint16_t = 0x19a;
pub const KEY_BREAK:             uint16_t = 0x19b;
pub const KEY_PREVIOUS:          uint16_t = 0x19c;
pub const KEY_DIGITS:            uint16_t = 0x19d;
pub const KEY_TEEN:              uint16_t = 0x19e;
pub const KEY_TWEN:              uint16_t = 0x19f;
pub const KEY_VIDEOPHONE:        uint16_t = 0x1a0; /* Media Select Video Phone */
pub const KEY_GAMES:             uint16_t = 0x1a1; /* Media Select Games */
pub const KEY_ZOOMIN:            uint16_t = 0x1a2; /* AC Zoom In */
pub const KEY_ZOOMOUT:           uint16_t = 0x1a3; /* AC Zoom Out */
pub const KEY_ZOOMRESET:         uint16_t = 0x1a4; /* AC Zoom */
pub const KEY_WORDPROCESSOR:     uint16_t = 0x1a5; /* AL Word Processor */
pub const KEY_EDITOR:            uint16_t = 0x1a6; /* AL Text Editor */
pub const KEY_SPREADSHEET:       uint16_t = 0x1a7; /* AL Spreadsheet */
pub const KEY_GRAPHICSEDITOR:    uint16_t = 0x1a8; /* AL Graphics Editor */
pub const KEY_PRESENTATION:      uint16_t = 0x1a9; /* AL Presentation App */
pub const KEY_DATABASE:          uint16_t = 0x1aa; /* AL Database App */
pub const KEY_NEWS:              uint16_t = 0x1ab; /* AL Newsreader */
pub const KEY_VOICEMAIL:         uint16_t = 0x1ac; /* AL Voicemail */
pub const KEY_ADDRESSBOOK:       uint16_t = 0x1ad; /* AL Contacts/Address Book */
pub const KEY_MESSENGER:         uint16_t = 0x1ae; /* AL Instant Messaging */
pub const KEY_DISPLAYTOGGLE:     uint16_t = 0x1af; /* Turn display : uint16_t = LCD on and off */
pub const KEY_BRIGHTNESS_TOGGLE: uint16_t = KEY_DISPLAYTOGGLE;
pub const KEY_SPELLCHECK:        uint16_t = 0x1b0; /* AL Spell Check */
pub const KEY_LOGOFF:            uint16_t = 0x1b1; /* AL Logoff */

pub const KEY_DOLLAR: uint16_t = 0x1b2;
pub const KEY_EURO:   uint16_t = 0x1b3;

pub const KEY_FRAMEBACK:      uint16_t = 0x1b4; /* Consumer - transport controls */
pub const KEY_FRAMEFORWARD:   uint16_t = 0x1b5;
pub const KEY_CONTEXT_MENU:   uint16_t = 0x1b6; /* GenDesc - system context menu */
pub const KEY_MEDIA_REPEAT:   uint16_t = 0x1b7; /* Consumer - transport control */
pub const KEY_10CHANNELSUP:   uint16_t = 0x1b8; /* 10 channels up : uint16_t = 10+ */
pub const KEY_10CHANNELSDOWN: uint16_t = 0x1b9; /* 10 channels down : uint16_t = 10- */
pub const KEY_IMAGES:         uint16_t = 0x1ba; /* AL Image Browser */

pub const KEY_DEL_EOL:  uint16_t = 0x1c0;
pub const KEY_DEL_EOS:  uint16_t = 0x1c1;
pub const KEY_INS_LINE: uint16_t = 0x1c2;
pub const KEY_DEL_LINE: uint16_t = 0x1c3;

pub const KEY_FN:     uint16_t = 0x1d0;
pub const KEY_FN_ESC: uint16_t = 0x1d1;
pub const KEY_FN_F1:  uint16_t = 0x1d2;
pub const KEY_FN_F2:  uint16_t = 0x1d3;
pub const KEY_FN_F3:  uint16_t = 0x1d4;
pub const KEY_FN_F4:  uint16_t = 0x1d5;
pub const KEY_FN_F5:  uint16_t = 0x1d6;
pub const KEY_FN_F6:  uint16_t = 0x1d7;
pub const KEY_FN_F7:  uint16_t = 0x1d8;
pub const KEY_FN_F8:  uint16_t = 0x1d9;
pub const KEY_FN_F9:  uint16_t = 0x1da;
pub const KEY_FN_F10: uint16_t = 0x1db;
pub const KEY_FN_F11: uint16_t = 0x1dc;
pub const KEY_FN_F12: uint16_t = 0x1dd;
pub const KEY_FN_1:   uint16_t = 0x1de;
pub const KEY_FN_2:   uint16_t = 0x1df;
pub const KEY_FN_D:   uint16_t = 0x1e0;
pub const KEY_FN_E:   uint16_t = 0x1e1;
pub const KEY_FN_F:   uint16_t = 0x1e2;
pub const KEY_FN_S:   uint16_t = 0x1e3;
pub const KEY_FN_B:   uint16_t = 0x1e4;

pub const KEY_BRL_DOT1:  uint16_t = 0x1f1;
pub const KEY_BRL_DOT2:  uint16_t = 0x1f2;
pub const KEY_BRL_DOT3:  uint16_t = 0x1f3;
pub const KEY_BRL_DOT4:  uint16_t = 0x1f4;
pub const KEY_BRL_DOT5:  uint16_t = 0x1f5;
pub const KEY_BRL_DOT6:  uint16_t = 0x1f6;
pub const KEY_BRL_DOT7:  uint16_t = 0x1f7;
pub const KEY_BRL_DOT8:  uint16_t = 0x1f8;
pub const KEY_BRL_DOT9:  uint16_t = 0x1f9;
pub const KEY_BRL_DOT10: uint16_t = 0x1fa;

pub const KEY_NUMERIC_0:     uint16_t = 0x200; /* used by phones, remote controls, */
pub const KEY_NUMERIC_1:     uint16_t = 0x201; /* and other keypads */
pub const KEY_NUMERIC_2:     uint16_t = 0x202;
pub const KEY_NUMERIC_3:     uint16_t = 0x203;
pub const KEY_NUMERIC_4:     uint16_t = 0x204;
pub const KEY_NUMERIC_5:     uint16_t = 0x205;
pub const KEY_NUMERIC_6:     uint16_t = 0x206;
pub const KEY_NUMERIC_7:     uint16_t = 0x207;
pub const KEY_NUMERIC_8:     uint16_t = 0x208;
pub const KEY_NUMERIC_9:     uint16_t = 0x209;
pub const KEY_NUMERIC_STAR:  uint16_t = 0x20a;
pub const KEY_NUMERIC_POUND: uint16_t = 0x20b;
pub const KEY_NUMERIC_A:     uint16_t = 0x20c; /* Phone key A - HUT Telephony 0xb9 */
pub const KEY_NUMERIC_B:     uint16_t = 0x20d;
pub const KEY_NUMERIC_C:     uint16_t = 0x20e;
pub const KEY_NUMERIC_D:     uint16_t = 0x20f;

pub const KEY_CAMERA_FOCUS: uint16_t = 0x210;
pub const KEY_WPS_BUTTON:   uint16_t = 0x211; /* WiFi Protected Setup key */

pub const KEY_TOUCHPAD_TOGGLE: uint16_t = 0x212; /* Request switch touchpad on or off */
pub const KEY_TOUCHPAD_ON:     uint16_t = 0x213;
pub const KEY_TOUCHPAD_OFF:    uint16_t = 0x214;

pub const KEY_CAMERA_ZOOMIN:  uint16_t = 0x215;
pub const KEY_CAMERA_ZOOMOUT: uint16_t = 0x216;
pub const KEY_CAMERA_UP:      uint16_t = 0x217;
pub const KEY_CAMERA_DOWN:    uint16_t = 0x218;
pub const KEY_CAMERA_LEFT:    uint16_t = 0x219;
pub const KEY_CAMERA_RIGHT:   uint16_t = 0x21a;

pub const KEY_ATTENDANT_ON:     uint16_t = 0x21b;
pub const KEY_ATTENDANT_OFF:    uint16_t = 0x21c;
pub const KEY_ATTENDANT_TOGGLE: uint16_t = 0x21d; /* Attendant call on or off */
pub const KEY_LIGHTS_TOGGLE:    uint16_t = 0x21e; /* Reading light on or off */

pub const BTN_DPAD_UP:    uint16_t = 0x220;
pub const BTN_DPAD_DOWN:  uint16_t = 0x221;
pub const BTN_DPAD_LEFT:  uint16_t = 0x222;
pub const BTN_DPAD_RIGHT: uint16_t = 0x223;

pub const KEY_ALS_TOGGLE: uint16_t = 0x230; /* Ambient light sensor */

pub const KEY_BUTTONCONFIG: uint16_t = 0x240;  /* AL Button Configuration */
pub const KEY_TASKMANAGER:  uint16_t = 0x241;  /* AL Task/Project Manager */
pub const KEY_JOURNAL:      uint16_t = 0x242;  /* AL Log/Journal/Timecard */
pub const KEY_CONTROLPANEL: uint16_t = 0x243;  /* AL Control Panel */
pub const KEY_APPSELECT:    uint16_t = 0x244;  /* AL Select Task/Application */
pub const KEY_SCREENSAVER:  uint16_t = 0x245;  /* AL Screen Saver */
pub const KEY_VOICECOMMAND: uint16_t = 0x246;  /* Listening Voice Command */

pub const KEY_BRIGHTNESS_MIN: uint16_t = 0x250; /* Set Brightness to Minimum */
pub const KEY_BRIGHTNESS_MAX: uint16_t = 0x251; /* Set Brightness to Maximum */

pub const KEY_KBDINPUTASSIST_PREV:      uint16_t = 0x260;
pub const KEY_KBDINPUTASSIST_NEXT:      uint16_t = 0x261;
pub const KEY_KBDINPUTASSIST_PREVGROUP: uint16_t = 0x262;
pub const KEY_KBDINPUTASSIST_NEXTGROUP: uint16_t = 0x263;
pub const KEY_KBDINPUTASSIST_ACCEPT:    uint16_t = 0x264;
pub const KEY_KBDINPUTASSIST_CANCEL:    uint16_t = 0x265;

pub const BTN_TRIGGER_HAPPY:   uint16_t = 0x2c0;
pub const BTN_TRIGGER_HAPPY1:  uint16_t = 0x2c0;
pub const BTN_TRIGGER_HAPPY2:  uint16_t = 0x2c1;
pub const BTN_TRIGGER_HAPPY3:  uint16_t = 0x2c2;
pub const BTN_TRIGGER_HAPPY4:  uint16_t = 0x2c3;
pub const BTN_TRIGGER_HAPPY5:  uint16_t = 0x2c4;
pub const BTN_TRIGGER_HAPPY6:  uint16_t = 0x2c5;
pub const BTN_TRIGGER_HAPPY7:  uint16_t = 0x2c6;
pub const BTN_TRIGGER_HAPPY8:  uint16_t = 0x2c7;
pub const BTN_TRIGGER_HAPPY9:  uint16_t = 0x2c8;
pub const BTN_TRIGGER_HAPPY10: uint16_t = 0x2c9;
pub const BTN_TRIGGER_HAPPY11: uint16_t = 0x2ca;
pub const BTN_TRIGGER_HAPPY12: uint16_t = 0x2cb;
pub const BTN_TRIGGER_HAPPY13: uint16_t = 0x2cc;
pub const BTN_TRIGGER_HAPPY14: uint16_t = 0x2cd;
pub const BTN_TRIGGER_HAPPY15: uint16_t = 0x2ce;
pub const BTN_TRIGGER_HAPPY16: uint16_t = 0x2cf;
pub const BTN_TRIGGER_HAPPY17: uint16_t = 0x2d0;
pub const BTN_TRIGGER_HAPPY18: uint16_t = 0x2d1;
pub const BTN_TRIGGER_HAPPY19: uint16_t = 0x2d2;
pub const BTN_TRIGGER_HAPPY20: uint16_t = 0x2d3;
pub const BTN_TRIGGER_HAPPY21: uint16_t = 0x2d4;
pub const BTN_TRIGGER_HAPPY22: uint16_t = 0x2d5;
pub const BTN_TRIGGER_HAPPY23: uint16_t = 0x2d6;
pub const BTN_TRIGGER_HAPPY24: uint16_t = 0x2d7;
pub const BTN_TRIGGER_HAPPY25: uint16_t = 0x2d8;
pub const BTN_TRIGGER_HAPPY26: uint16_t = 0x2d9;
pub const BTN_TRIGGER_HAPPY27: uint16_t = 0x2da;
pub const BTN_TRIGGER_HAPPY28: uint16_t = 0x2db;
pub const BTN_TRIGGER_HAPPY29: uint16_t = 0x2dc;
pub const BTN_TRIGGER_HAPPY30: uint16_t = 0x2dd;
pub const BTN_TRIGGER_HAPPY31: uint16_t = 0x2de;
pub const BTN_TRIGGER_HAPPY32: uint16_t = 0x2df;
pub const BTN_TRIGGER_HAPPY33: uint16_t = 0x2e0;
pub const BTN_TRIGGER_HAPPY34: uint16_t = 0x2e1;
pub const BTN_TRIGGER_HAPPY35: uint16_t = 0x2e2;
pub const BTN_TRIGGER_HAPPY36: uint16_t = 0x2e3;
pub const BTN_TRIGGER_HAPPY37: uint16_t = 0x2e4;
pub const BTN_TRIGGER_HAPPY38: uint16_t = 0x2e5;
pub const BTN_TRIGGER_HAPPY39: uint16_t = 0x2e6;
pub const BTN_TRIGGER_HAPPY40: uint16_t = 0x2e7;

/* We avoid low common keys in module aliases so they don't get huge. */
pub const KEY_MIN_INTERESTING: uint16_t = KEY_MUTE;
pub const KEY_MAX:             uint16_t = 0x2ff;
pub const KEY_CNT:             uint16_t = KEY_MAX + 1;

/*
 * Relative axes
 */

pub const REL_X:      uint16_t = 0x00;
pub const REL_Y:      uint16_t = 0x01;
pub const REL_Z:      uint16_t = 0x02;
pub const REL_RX:     uint16_t = 0x03;
pub const REL_RY:     uint16_t = 0x04;
pub const REL_RZ:     uint16_t = 0x05;
pub const REL_HWHEEL: uint16_t = 0x06;
pub const REL_DIAL:   uint16_t = 0x07;
pub const REL_WHEEL:  uint16_t = 0x08;
pub const REL_MISC:   uint16_t = 0x09;
pub const REL_MAX:    uint16_t = 0x0f;
pub const REL_CNT:    uint16_t = REL_MAX + 1;

/*
 * Absolute axes
 */

pub const ABS_X:          uint16_t = 0x00;
pub const ABS_Y:          uint16_t = 0x01;
pub const ABS_Z:          uint16_t = 0x02;
pub const ABS_RX:         uint16_t = 0x03;
pub const ABS_RY:         uint16_t = 0x04;
pub const ABS_RZ:         uint16_t = 0x05;
pub const ABS_THROTTLE:   uint16_t = 0x06;
pub const ABS_RUDDER:     uint16_t = 0x07;
pub const ABS_WHEEL:      uint16_t = 0x08;
pub const ABS_GAS:        uint16_t = 0x09;
pub const ABS_BRAKE:      uint16_t = 0x0a;
pub const ABS_HAT0X:      uint16_t = 0x10;
pub const ABS_HAT0Y:      uint16_t = 0x11;
pub const ABS_HAT1X:      uint16_t = 0x12;
pub const ABS_HAT1Y:      uint16_t = 0x13;
pub const ABS_HAT2X:      uint16_t = 0x14;
pub const ABS_HAT2Y:      uint16_t = 0x15;
pub const ABS_HAT3X:      uint16_t = 0x16;
pub const ABS_HAT3Y:      uint16_t = 0x17;
pub const ABS_PRESSURE:   uint16_t = 0x18;
pub const ABS_DISTANCE:   uint16_t = 0x19;
pub const ABS_TILT_X:     uint16_t = 0x1a;
pub const ABS_TILT_Y:     uint16_t = 0x1b;
pub const ABS_TOOL_WIDTH: uint16_t = 0x1c;

pub const ABS_VOLUME: uint16_t = 0x20;

pub const ABS_MISC: uint16_t = 0x28;

pub const ABS_MT_SLOT:        uint16_t = 0x2f; /* MT slot being modified */
pub const ABS_MT_TOUCH_MAJOR: uint16_t = 0x30; /* Major axis of touching ellipse */
pub const ABS_MT_TOUCH_MINOR: uint16_t = 0x31; /* Minor axis : uint16_t = omit if circular */
pub const ABS_MT_WIDTH_MAJOR: uint16_t = 0x32; /* Major axis of approaching ellipse */
pub const ABS_MT_WIDTH_MINOR: uint16_t = 0x33; /* Minor axis : uint16_t = omit if circular */
pub const ABS_MT_ORIENTATION: uint16_t = 0x34; /* Ellipse orientation */
pub const ABS_MT_POSITION_X:  uint16_t = 0x35; /* Center X touch position */
pub const ABS_MT_POSITION_Y:  uint16_t = 0x36; /* Center Y touch position */
pub const ABS_MT_TOOL_TYPE:   uint16_t = 0x37; /* Type of touching device */
pub const ABS_MT_BLOB_ID:     uint16_t = 0x38; /* Group a set of packets as a blob */
pub const ABS_MT_TRACKING_ID: uint16_t = 0x39; /* Unique ID of initiated contact */
pub const ABS_MT_PRESSURE:    uint16_t = 0x3a; /* Pressure on contact area */
pub const ABS_MT_DISTANCE:    uint16_t = 0x3b; /* Contact hover distance */
pub const ABS_MT_TOOL_X:      uint16_t = 0x3c; /* Center X tool position */
pub const ABS_MT_TOOL_Y:      uint16_t = 0x3d; /* Center Y tool position */


pub const ABS_MAX: uint16_t = 0x3f;
pub const ABS_CNT: uint16_t = ABS_MAX + 1;

/*
 * Switch events
 */

pub const SW_LID:                  uint16_t = 0x00;  /* set = lid shut */
pub const SW_TABLET_MODE:          uint16_t = 0x01;  /* set = tablet mode */
pub const SW_HEADPHONE_INSERT:     uint16_t = 0x02;  /* set = inserted */
pub const SW_RFKILL_ALL:           uint16_t = 0x03;  /* rfkill master switch, type "any" set = radio enabled */
pub const SW_RADIO:                uint16_t = SW_RFKILL_ALL; /* deprecated */
pub const SW_MICROPHONE_INSERT:    uint16_t = 0x04;  /* set = inserted */
pub const SW_DOCK:                 uint16_t = 0x05;  /* set = plugged into dock */
pub const SW_LINEOUT_INSERT:       uint16_t = 0x06;  /* set = inserted */
pub const SW_JACK_PHYSICAL_INSERT: uint16_t = 0x07;  /* set = mechanical switch set */
pub const SW_VIDEOOUT_INSERT:      uint16_t = 0x08;  /* set = inserted */
pub const SW_CAMERA_LENS_COVER:    uint16_t = 0x09;  /* set = lens covered */
pub const SW_KEYPAD_SLIDE:         uint16_t = 0x0a;  /* set = keypad slide out */
pub const SW_FRONT_PROXIMITY:      uint16_t = 0x0b;  /* set = front proximity sensor active */
pub const SW_ROTATE_LOCK:          uint16_t = 0x0c;  /* set = rotate locked/disabled */
pub const SW_LINEIN_INSERT:        uint16_t = 0x0d;  /* set = inserted */
pub const SW_MUTE_DEVICE:          uint16_t = 0x0e;  /* set = device disabled */
pub const SW_MAX:                  uint16_t = 0x0f;
pub const SW_CNT:                  uint16_t = SW_MAX + 1;

/*
 * Misc events
 */

pub const MSC_SERIAL:    uint16_t = 0x00;
pub const MSC_PULSELED:  uint16_t = 0x01;
pub const MSC_GESTURE:   uint16_t = 0x02;
pub const MSC_RAW:       uint16_t = 0x03;
pub const MSC_SCAN:      uint16_t = 0x04;
pub const MSC_TIMESTAMP: uint16_t = 0x05;
pub const MSC_MAX:       uint16_t = 0x07;
pub const MSC_CNT:       uint16_t = MSC_MAX + 1;

/*
 * LEDs
 */

pub const LED_NUML:     uint16_t = 0x00;
pub const LED_CAPSL:    uint16_t = 0x01;
pub const LED_SCROLLL:  uint16_t = 0x02;
pub const LED_COMPOSE:  uint16_t = 0x03;
pub const LED_KANA:     uint16_t = 0x04;
pub const LED_SLEEP:    uint16_t = 0x05;
pub const LED_SUSPEND:  uint16_t = 0x06;
pub const LED_MUTE:     uint16_t = 0x07;
pub const LED_MISC:     uint16_t = 0x08;
pub const LED_MAIL:     uint16_t = 0x09;
pub const LED_CHARGING: uint16_t = 0x0a;
pub const LED_MAX:      uint16_t = 0x0f;
pub const LED_CNT:      uint16_t = LED_MAX + 1;

/*
 * Autorepeat values
 */

pub const REP_DELAY:  uint16_t = 0x00;
pub const REP_PERIOD: uint16_t = 0x01;
pub const REP_MAX:    uint16_t = 0x01;
pub const REP_CNT:    uint16_t = REP_MAX + 1;

/*
 * Sounds
 */

pub const SND_CLICK: uint16_t = 0x00;
pub const SND_BELL:  uint16_t = 0x01;
pub const SND_TONE:  uint16_t = 0x02;
pub const SND_MAX:   uint16_t = 0x07;
pub const SND_CNT:   uint16_t = SND_MAX + 1;
