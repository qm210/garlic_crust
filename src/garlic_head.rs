use crate::garlic_crust::ZERO_SAMPLE;
use super::garlic_crust::*;

// TODO: track could be a byte array. if that saves us something?

// PUT GARLIC_EXTRACT HERE >>>>>>>>

pub const SECONDS: TimeFloat = 30.530;

const SEQUENCE_0: [SeqEvent; 129] = [
    SeqEvent {pos: 22050, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 31706, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 41363, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 51020, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 60677, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 70334, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 79991, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 89648, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 99305, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 108962, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 118619, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 128276, message: SeqMsg::NoteOn(39, 113) },
    SeqEvent {pos: 137933, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 147590, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 157247, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 166903, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 176560, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 186217, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 195874, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 205531, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 215188, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 224845, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 234502, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 244159, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 253816, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 263473, message: SeqMsg::NoteOn(49, 113) },
    SeqEvent {pos: 273130, message: SeqMsg::NoteOn(56, 113) },
    SeqEvent {pos: 282787, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 292444, message: SeqMsg::NoteOn(58, 113) },
    SeqEvent {pos: 302100, message: SeqMsg::NoteOn(52, 113) },
    SeqEvent {pos: 311757, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 321414, message: SeqMsg::NoteOn(55, 113) },
    SeqEvent {pos: 331071, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 340728, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 350385, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 360042, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 369699, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 379356, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 389013, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 398670, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 408327, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 417984, message: SeqMsg::NoteOn(39, 113) },
    SeqEvent {pos: 427641, message: SeqMsg::NoteOn(47, 113) },
    SeqEvent {pos: 437297, message: SeqMsg::NoteOn(42, 113) },
    SeqEvent {pos: 446954, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 456611, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 466268, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 475925, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 485582, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 495239, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 504896, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 514553, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 524210, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 533867, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 543524, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 553181, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 562838, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 572495, message: SeqMsg::NoteOn(49, 113) },
    SeqEvent {pos: 582151, message: SeqMsg::NoteOn(56, 113) },
    SeqEvent {pos: 591808, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 601465, message: SeqMsg::NoteOn(58, 113) },
    SeqEvent {pos: 611122, message: SeqMsg::NoteOn(52, 113) },
    SeqEvent {pos: 620779, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 630436, message: SeqMsg::NoteOn(55, 113) },
    SeqEvent {pos: 640093, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 649750, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 659407, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 669064, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 678721, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 688378, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 698035, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 707692, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 717349, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 727005, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 736662, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 746319, message: SeqMsg::NoteOn(39, 113) },
    SeqEvent {pos: 755976, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 765633, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 775290, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 784947, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 794604, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 804261, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 813918, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 823575, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 833232, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 842889, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 852545, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 862202, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 871859, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 881516, message: SeqMsg::NoteOn(49, 113) },
    SeqEvent {pos: 891173, message: SeqMsg::NoteOn(56, 113) },
    SeqEvent {pos: 900830, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 910487, message: SeqMsg::NoteOn(58, 113) },
    SeqEvent {pos: 920144, message: SeqMsg::NoteOn(52, 113) },
    SeqEvent {pos: 929801, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 939458, message: SeqMsg::NoteOn(55, 113) },
    SeqEvent {pos: 949115, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 958772, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 968429, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 978086, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 987743, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 997399, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 1007056, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 1016713, message: SeqMsg::NoteOn(36, 113) },
    SeqEvent {pos: 1026370, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 1036027, message: SeqMsg::NoteOn(39, 113) },
    SeqEvent {pos: 1045684, message: SeqMsg::NoteOn(47, 113) },
    SeqEvent {pos: 1055341, message: SeqMsg::NoteOn(42, 113) },
    SeqEvent {pos: 1064998, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 1074655, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 1084312, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 1093969, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 1103626, message: SeqMsg::NoteOn(53, 113) },
    SeqEvent {pos: 1113283, message: SeqMsg::NoteOn(41, 113) },
    SeqEvent {pos: 1122940, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 1132597, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 1142253, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 1151910, message: SeqMsg::NoteOn(44, 113) },
    SeqEvent {pos: 1161567, message: SeqMsg::NoteOn(48, 113) },
    SeqEvent {pos: 1171224, message: SeqMsg::NoteOn(40, 113) },
    SeqEvent {pos: 1180881, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 1190538, message: SeqMsg::NoteOn(49, 113) },
    SeqEvent {pos: 1200195, message: SeqMsg::NoteOn(56, 113) },
    SeqEvent {pos: 1209852, message: SeqMsg::NoteOn(51, 113) },
    SeqEvent {pos: 1219509, message: SeqMsg::NoteOn(58, 113) },
    SeqEvent {pos: 1229166, message: SeqMsg::NoteOn(52, 113) },
    SeqEvent {pos: 1238823, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 1248480, message: SeqMsg::NoteOn(55, 113) },
    SeqEvent {pos: 1258137, message: SeqMsg::NoteOff },
];

const SEQUENCE_1: [SeqEvent; 133] = [
    SeqEvent {pos: 22050, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {pos: 31706, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 41363, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 51020, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 60677, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 70334, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 79991, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 89648, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 99305, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 108962, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 118619, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 128276, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 137933, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 147590, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 157247, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 166903, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 176560, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {pos: 186217, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 195874, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 205531, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 215188, message: SeqMsg::NoteOn(71, 113) },
    SeqEvent {pos: 224845, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 234502, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 244159, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 253816, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 263473, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 273130, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 282787, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 292444, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 302100, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 311757, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 321414, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 331071, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 340728, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 350385, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 360042, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 369699, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 379356, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 389013, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 398670, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 408327, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 417984, message: SeqMsg::NoteOn(75, 113) },
    SeqEvent {pos: 427641, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 437297, message: SeqMsg::NoteOn(75, 113) },
    SeqEvent {pos: 446954, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 456611, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 466268, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 475925, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 485582, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 495239, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 504896, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 514553, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 524210, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 533867, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 543524, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 553181, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 562838, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 572495, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 582151, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 591808, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 601465, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 611122, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 620779, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 630436, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 640093, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {pos: 659407, message: SeqMsg::NoteOff },
    SeqEvent {pos: 669064, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 678721, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 688378, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 698035, message: SeqMsg::NoteOn(75, 113) },
    SeqEvent {pos: 717349, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 727005, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 736662, message: SeqMsg::NoteOff },
    SeqEvent {pos: 746319, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 755976, message: SeqMsg::NoteOn(79, 113) },
    SeqEvent {pos: 794604, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {pos: 799432, message: SeqMsg::NoteOff },
    SeqEvent {pos: 804261, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 813918, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 823575, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 833232, message: SeqMsg::NoteOn(71, 113) },
    SeqEvent {pos: 842889, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 852545, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 862202, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 871859, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 876688, message: SeqMsg::NoteOff },
    SeqEvent {pos: 881516, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 886345, message: SeqMsg::NoteOff },
    SeqEvent {pos: 891173, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 896002, message: SeqMsg::NoteOff },
    SeqEvent {pos: 900830, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 905659, message: SeqMsg::NoteOff },
    SeqEvent {pos: 910487, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 915316, message: SeqMsg::NoteOff },
    SeqEvent {pos: 920144, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 924972, message: SeqMsg::NoteOff },
    SeqEvent {pos: 929801, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 934629, message: SeqMsg::NoteOff },
    SeqEvent {pos: 939458, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 944286, message: SeqMsg::NoteOff },
    SeqEvent {pos: 949115, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 958772, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 968429, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 978086, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 987743, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 997399, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 1007056, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 1016713, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 1026370, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 1036027, message: SeqMsg::NoteOn(75, 113) },
    SeqEvent {pos: 1045684, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 1055341, message: SeqMsg::NoteOn(75, 113) },
    SeqEvent {pos: 1064998, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 1074655, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 1084312, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 1093969, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 1103626, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 1113283, message: SeqMsg::NoteOn(65, 113) },
    SeqEvent {pos: 1122940, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 1132597, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 1142253, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 1151910, message: SeqMsg::NoteOn(60, 113) },
    SeqEvent {pos: 1161567, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 1171224, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 1180881, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 1190538, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 1200195, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 1209852, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 1219509, message: SeqMsg::NoteOn(68, 113) },
    SeqEvent {pos: 1229166, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 1238823, message: SeqMsg::NoteOn(67, 113) },
    SeqEvent {pos: 1248480, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 1258137, message: SeqMsg::NoteOff },
];

const SEQUENCE_2: [SeqEvent; 80] = [
    SeqEvent {pos: 379356, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 389013, message: SeqMsg::NoteOff },
    SeqEvent {pos: 727005, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {pos: 746319, message: SeqMsg::NoteOff },
    SeqEvent {pos: 765633, message: SeqMsg::NoteOn(64, 113) },
    SeqEvent {pos: 775290, message: SeqMsg::NoteOn(70, 113) },
    SeqEvent {pos: 784947, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 794604, message: SeqMsg::NoteOff },
    SeqEvent {pos: 804261, message: SeqMsg::NoteOn(84, 113) },
    SeqEvent {pos: 809089, message: SeqMsg::NoteOff },
    SeqEvent {pos: 813918, message: SeqMsg::NoteOn(80, 113) },
    SeqEvent {pos: 818746, message: SeqMsg::NoteOff },
    SeqEvent {pos: 823575, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {pos: 828403, message: SeqMsg::NoteOff },
    SeqEvent {pos: 833232, message: SeqMsg::NoteOn(85, 113) },
    SeqEvent {pos: 838060, message: SeqMsg::NoteOff },
    SeqEvent {pos: 842889, message: SeqMsg::NoteOn(80, 113) },
    SeqEvent {pos: 847717, message: SeqMsg::NoteOff },
    SeqEvent {pos: 852545, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {pos: 857374, message: SeqMsg::NoteOff },
    SeqEvent {pos: 862202, message: SeqMsg::NoteOn(76, 113) },
    SeqEvent {pos: 867031, message: SeqMsg::NoteOff },
    SeqEvent {pos: 874274, message: SeqMsg::NoteOn(80, 113) },
    SeqEvent {pos: 879102, message: SeqMsg::NoteOff },
    SeqEvent {pos: 883931, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {pos: 888759, message: SeqMsg::NoteOff },
    SeqEvent {pos: 893587, message: SeqMsg::NoteOn(79, 113) },
    SeqEvent {pos: 898416, message: SeqMsg::NoteOff },
    SeqEvent {pos: 903244, message: SeqMsg::NoteOn(80, 113) },
    SeqEvent {pos: 908073, message: SeqMsg::NoteOff },
    SeqEvent {pos: 912901, message: SeqMsg::NoteOn(84, 113) },
    SeqEvent {pos: 917730, message: SeqMsg::NoteOff },
    SeqEvent {pos: 922558, message: SeqMsg::NoteOn(79, 113) },
    SeqEvent {pos: 927387, message: SeqMsg::NoteOff },
    SeqEvent {pos: 932215, message: SeqMsg::NoteOn(84, 113) },
    SeqEvent {pos: 937044, message: SeqMsg::NoteOff },
    SeqEvent {pos: 941872, message: SeqMsg::NoteOn(82, 113) },
    SeqEvent {pos: 946701, message: SeqMsg::NoteOff },
    SeqEvent {pos: 949115, message: SeqMsg::NoteOn(77, 113) },
    SeqEvent {pos: 968429, message: SeqMsg::NoteOff },
    SeqEvent {pos: 978086, message: SeqMsg::NoteOn(79, 113) },
    SeqEvent {pos: 997399, message: SeqMsg::NoteOn(72, 113) },
    SeqEvent {pos: 1007056, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1026370, message: SeqMsg::NoteOn(80, 113) },
    SeqEvent {pos: 1045684, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1055341, message: SeqMsg::NoteOn(82, 113) },
    SeqEvent {pos: 1079483, message: SeqMsg::NoteOn(84, 113) },
    SeqEvent {pos: 1103626, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1106040, message: SeqMsg::NoteOn(96, 113) },
    SeqEvent {pos: 1108454, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1115697, message: SeqMsg::NoteOn(89, 113) },
    SeqEvent {pos: 1118111, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1125354, message: SeqMsg::NoteOn(92, 113) },
    SeqEvent {pos: 1127768, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1135011, message: SeqMsg::NoteOn(96, 113) },
    SeqEvent {pos: 1137425, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1144668, message: SeqMsg::NoteOn(91, 113) },
    SeqEvent {pos: 1147082, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1154325, message: SeqMsg::NoteOn(84, 113) },
    SeqEvent {pos: 1156739, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1163981, message: SeqMsg::NoteOn(88, 113) },
    SeqEvent {pos: 1166396, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1173638, message: SeqMsg::NoteOn(92, 113) },
    SeqEvent {pos: 1176053, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1183295, message: SeqMsg::NoteOn(84, 113) },
    SeqEvent {pos: 1185710, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1192952, message: SeqMsg::NoteOn(88, 113) },
    SeqEvent {pos: 1195367, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1202609, message: SeqMsg::NoteOn(91, 113) },
    SeqEvent {pos: 1205024, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1212266, message: SeqMsg::NoteOn(94, 113) },
    SeqEvent {pos: 1214680, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1221923, message: SeqMsg::NoteOn(82, 113) },
    SeqEvent {pos: 1224337, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1231580, message: SeqMsg::NoteOn(94, 113) },
    SeqEvent {pos: 1233994, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1241237, message: SeqMsg::NoteOn(84, 113) },
    SeqEvent {pos: 1243651, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1250894, message: SeqMsg::NoteOn(88, 113) },
    SeqEvent {pos: 1253308, message: SeqMsg::NoteOff },
];

const SEQUENCE_3: [SeqEvent; 6] = [
    SeqEvent {pos: 1183295, message: SeqMsg::NoteOn(94, 113) },
    SeqEvent {pos: 1185710, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1221923, message: SeqMsg::NoteOn(92, 113) },
    SeqEvent {pos: 1224337, message: SeqMsg::NoteOff },
    SeqEvent {pos: 1241237, message: SeqMsg::NoteOn(91, 113) },
    SeqEvent {pos: 1243651, message: SeqMsg::NoteOff },
];

// <<<<<<<< PUT GARLIC_EXTRACT HERE

pub const BLOCK_SIZE: usize = 512;
const MASTER_BLOCK_FACTOR: usize = 4; // my stolen freeverb needs this for now
pub const MASTER_BLOCK_SIZE: usize = BLOCK_SIZE * MASTER_BLOCK_FACTOR;
const MASTER_BLOCK_NUMBER: usize = ((SAMPLERATE * SECONDS) as usize / MASTER_BLOCK_SIZE) + 1;
pub const SAMPLES: usize = MASTER_BLOCK_NUMBER * MASTER_BLOCK_SIZE;

pub type BlockArray = [Sample; BLOCK_SIZE];
pub type MasterBlockArray = [Sample; MASTER_BLOCK_SIZE];
pub type MasterBlockMono = [MonoSample; MASTER_BLOCK_SIZE];
pub type StereoTrack = [MonoSample; 2 * SAMPLES];

pub const EMPTY_BLOCKARRAY: BlockArray = [ZERO_SAMPLE; BLOCK_SIZE];

mod garlic_clove1;
mod garlic_master;

pub unsafe fn render_track(data: &mut StereoTrack) {
    let mut garlic_master = garlic_master::GarlicMaster::new(); // here would configuration go

    // we need global initialization, one per clove and each their sequence
    let clove1_config1 = garlic_clove1::create_config1("default");
    let clove1_config2 = garlic_clove1::create_config2("default");
    let mut clove1_state0 = garlic_clove1::create_state(&clove1_config1, &clove1_config2);
    let mut clove1_state1 = garlic_clove1::create_state(&clove1_config1, &clove1_config2);
    let mut clove1_state2 = garlic_clove1::create_state(&clove1_config1, &clove1_config2);
    let mut clove1_state3 = garlic_clove1::create_state(&clove1_config1, &clove1_config2);

    let mut master_block_offset = 0;
    let mut block_offset = 0;

    while master_block_offset < 2 * SAMPLES {

        for master_piece in 0 .. MASTER_BLOCK_FACTOR {
            garlic_clove1::process(&SEQUENCE_0, block_offset, &mut clove1_state0);
            garlic_clove1::process(&SEQUENCE_1, block_offset, &mut clove1_state1);
            garlic_clove1::process(&SEQUENCE_2, block_offset, &mut clove1_state2);
            garlic_clove1::process(&SEQUENCE_3, block_offset, &mut clove1_state3);

            for sample in 0 .. BLOCK_SIZE {
                let master_sample = sample + master_piece * BLOCK_SIZE;

                garlic_master.add_at(master_sample, clove1_state0.output[sample]);
                garlic_master.add_at(master_sample, clove1_state1.output[sample]);
                garlic_master.add_at(master_sample, clove1_state2.output[sample]);
                garlic_master.add_at(master_sample, clove1_state3.output[sample]);

                garlic_master.process(master_sample);
            }
            block_offset += BLOCK_SIZE;
        }

        garlic_master.write(data, master_block_offset);
        // super::printf("Block finished: %d %d .. %d\n\0".as_ptr(), master_block_offset, block_offset, SAMPLES);

        master_block_offset += 2 * MASTER_BLOCK_SIZE;
    }

    let mut clipping_count = 0;
    let mut max_sample = 0.;
    let mut min_sample = 0.;

    for sample in 0 .. 2 * SAMPLES {
        if data[sample] > 1. || data[sample] < -1.
        || data[sample] > 1. || data[sample] < -1. {
            clipping_count += 1;
        }
        if data[sample] > max_sample {
            max_sample = data[sample];
        }
        if data[sample] < min_sample {
            min_sample = data[sample];
        }
    }

    super::printf("Real duration: %.3fs\n\0".as_ptr(), SAMPLES as f64 / SAMPLERATE as f64);
    super::printf("Range: %.3f .. %.3f\n\0".as_ptr(), min_sample as f64, max_sample as f64);
    super::printf("Clipping counter: %d\n\0".as_ptr(), clipping_count);
}
