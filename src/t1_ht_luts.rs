use super::openjpeg::*;

pub const vlc_tbl0: [OPJ_UINT16; 1024] = [
  0x23, 0xa5, 0x43, 0x66, 0x83, 0xa8ee, 0x14, 0xd8df, 0x23, 0x10be, 0x43, 0xf5ff, 0x83, 0x207e,
  0x55, 0x515f, 0x23, 0x35, 0x43, 0x444e, 0x83, 0xc4ce, 0x14, 0xcccf, 0x23, 0xe2fe, 0x43, 0x99ff,
  0x83, 0x96, 0xc5, 0x313f, 0x23, 0xa5, 0x43, 0x445e, 0x83, 0xc8ce, 0x14, 0x11df, 0x23, 0xf4fe,
  0x43, 0xfcff, 0x83, 0x9e, 0x55, 0x77, 0x23, 0x35, 0x43, 0xf1ff, 0x83, 0x88ae, 0x14, 0xb7, 0x23,
  0xf8fe, 0x43, 0xe4ef, 0x83, 0x888e, 0xc5, 0x111f, 0x23, 0xa5, 0x43, 0x66, 0x83, 0xa8ee, 0x14,
  0x54df, 0x23, 0x10be, 0x43, 0x22ef, 0x83, 0x207e, 0x55, 0x227f, 0x23, 0x35, 0x43, 0x444e, 0x83,
  0xc4ce, 0x14, 0x11bf, 0x23, 0xe2fe, 0x43, 0xf7, 0x83, 0x96, 0xc5, 0x223f, 0x23, 0xa5, 0x43,
  0x445e, 0x83, 0xc8ce, 0x14, 0xd7, 0x23, 0xf4fe, 0x43, 0xbaff, 0x83, 0x9e, 0x55, 0x6f, 0x23, 0x35,
  0x43, 0xe6ff, 0x83, 0x88ae, 0x14, 0xa2af, 0x23, 0xf8fe, 0x43, 0xe7, 0x83, 0x888e, 0xc5, 0x222f,
  0x2, 0xc5, 0x84, 0x207e, 0x2, 0xc4ce, 0x24, 0xf7, 0x2, 0xa2fe, 0x44, 0x56, 0x2, 0x9e, 0x14, 0xd7,
  0x2, 0x10be, 0x84, 0x66, 0x2, 0x88ae, 0x24, 0x11df, 0x2, 0xa8ee, 0x44, 0x36, 0x2, 0x888e, 0x14,
  0x111f, 0x2, 0xc5, 0x84, 0x6e, 0x2, 0x88ce, 0x24, 0x88ff, 0x2, 0xb8fe, 0x44, 0x444e, 0x2, 0x96,
  0x14, 0xb7, 0x2, 0xe4fe, 0x84, 0x445e, 0x2, 0xa6, 0x24, 0xe7, 0x2, 0x54de, 0x44, 0x222e, 0x2,
  0x3e, 0x14, 0x77, 0x2, 0xc5, 0x84, 0x207e, 0x2, 0xc4ce, 0x24, 0xf1ff, 0x2, 0xa2fe, 0x44, 0x56,
  0x2, 0x9e, 0x14, 0x11bf, 0x2, 0x10be, 0x84, 0x66, 0x2, 0x88ae, 0x24, 0x22ef, 0x2, 0xa8ee, 0x44,
  0x36, 0x2, 0x888e, 0x14, 0x227f, 0x2, 0xc5, 0x84, 0x6e, 0x2, 0x88ce, 0x24, 0xe4ef, 0x2, 0xb8fe,
  0x44, 0x444e, 0x2, 0x96, 0x14, 0xa2af, 0x2, 0xe4fe, 0x84, 0x445e, 0x2, 0xa6, 0x24, 0xd8df, 0x2,
  0x54de, 0x44, 0x222e, 0x2, 0x3e, 0x14, 0x515f, 0x2, 0x55, 0x84, 0x66, 0x2, 0x88de, 0x24, 0x32ff,
  0x2, 0x11fe, 0x44, 0x444e, 0x2, 0xae, 0x14, 0xb7, 0x2, 0x317e, 0x84, 0x515e, 0x2, 0xc6, 0x24,
  0xd7, 0x2, 0x20ee, 0x44, 0x111e, 0x2, 0x9e, 0x14, 0x77, 0x2, 0x55, 0x84, 0x545e, 0x2, 0x44ce,
  0x24, 0xe7, 0x2, 0xf1fe, 0x44, 0x36, 0x2, 0xa6, 0x14, 0x555f, 0x2, 0x74fe, 0x84, 0x113e, 0x2,
  0x20be, 0x24, 0x747f, 0x2, 0xc4de, 0x44, 0xf8ff, 0x2, 0x96, 0x14, 0x222f, 0x2, 0x55, 0x84, 0x66,
  0x2, 0x88de, 0x24, 0xf7, 0x2, 0x11fe, 0x44, 0x444e, 0x2, 0xae, 0x14, 0x888f, 0x2, 0x317e, 0x84,
  0x515e, 0x2, 0xc6, 0x24, 0xc8cf, 0x2, 0x20ee, 0x44, 0x111e, 0x2, 0x9e, 0x14, 0x6f, 0x2, 0x55,
  0x84, 0x545e, 0x2, 0x44ce, 0x24, 0xd1df, 0x2, 0xf1fe, 0x44, 0x36, 0x2, 0xa6, 0x14, 0x227f, 0x2,
  0x74fe, 0x84, 0x113e, 0x2, 0x20be, 0x24, 0x22bf, 0x2, 0xc4de, 0x44, 0x22ef, 0x2, 0x96, 0x14,
  0x323f, 0x3, 0xd4de, 0xf4fd, 0xfcff, 0x14, 0x113e, 0x55, 0x888f, 0x3, 0x32be, 0x85, 0xe7, 0x25,
  0x515e, 0xaafe, 0x727f, 0x3, 0x44ce, 0xf8fd, 0x44ef, 0x14, 0x647e, 0x45, 0xa2af, 0x3, 0xa6,
  0x555d, 0x99df, 0xf1fd, 0x36, 0xf5fe, 0x626f, 0x3, 0xd1de, 0xf4fd, 0xe6ff, 0x14, 0x717e, 0x55,
  0xb1bf, 0x3, 0x88ae, 0x85, 0xd5df, 0x25, 0x444e, 0xf2fe, 0x667f, 0x3, 0xc6, 0xf8fd, 0xe2ef, 0x14,
  0x545e, 0x45, 0x119f, 0x3, 0x96, 0x555d, 0xc8cf, 0xf1fd, 0x111e, 0xc8ee, 0x67, 0x3, 0xd4de,
  0xf4fd, 0xf3ff, 0x14, 0x113e, 0x55, 0x11bf, 0x3, 0x32be, 0x85, 0xd8df, 0x25, 0x515e, 0xaafe,
  0x222f, 0x3, 0x44ce, 0xf8fd, 0xf7, 0x14, 0x647e, 0x45, 0x989f, 0x3, 0xa6, 0x555d, 0xd7, 0xf1fd,
  0x36, 0xf5fe, 0x446f, 0x3, 0xd1de, 0xf4fd, 0xb9ff, 0x14, 0x717e, 0x55, 0xb7, 0x3, 0x88ae, 0x85,
  0xdcdf, 0x25, 0x444e, 0xf2fe, 0x77, 0x3, 0xc6, 0xf8fd, 0xe4ef, 0x14, 0x545e, 0x45, 0x737f, 0x3,
  0x96, 0x555d, 0xb8bf, 0xf1fd, 0x111e, 0xc8ee, 0x323f, 0x2, 0xa5, 0x84, 0x407e, 0x2, 0x10de, 0x24,
  0x11df, 0x2, 0x72fe, 0x44, 0x56, 0x2, 0xa8ae, 0x14, 0xb2bf, 0x2, 0x96, 0x84, 0x66, 0x2, 0xc6,
  0x24, 0xe7, 0x2, 0xc8ee, 0x44, 0x222e, 0x2, 0x888e, 0x14, 0x77, 0x2, 0xa5, 0x84, 0x6e, 0x2,
  0x88ce, 0x24, 0xf7, 0x2, 0x91fe, 0x44, 0x36, 0x2, 0xa2ae, 0x14, 0xaaaf, 0x2, 0xb8fe, 0x84, 0x5e,
  0x2, 0xbe, 0x24, 0xc4cf, 0x2, 0x44ee, 0x44, 0xf4ff, 0x2, 0x223e, 0x14, 0x111f, 0x2, 0xa5, 0x84,
  0x407e, 0x2, 0x10de, 0x24, 0x99ff, 0x2, 0x72fe, 0x44, 0x56, 0x2, 0xa8ae, 0x14, 0xb7, 0x2, 0x96,
  0x84, 0x66, 0x2, 0xc6, 0x24, 0xd7, 0x2, 0xc8ee, 0x44, 0x222e, 0x2, 0x888e, 0x14, 0x444f, 0x2,
  0xa5, 0x84, 0x6e, 0x2, 0x88ce, 0x24, 0xe2ef, 0x2, 0x91fe, 0x44, 0x36, 0x2, 0xa2ae, 0x14, 0x447f,
  0x2, 0xb8fe, 0x84, 0x5e, 0x2, 0xbe, 0x24, 0x9f, 0x2, 0x44ee, 0x44, 0x76ff, 0x2, 0x223e, 0x14,
  0x313f, 0x3, 0xc6, 0x85, 0xd9ff, 0xf2fd, 0x647e, 0xf1fe, 0x99bf, 0x3, 0xa2ae, 0x25, 0x66ef,
  0xf4fd, 0x56, 0xe2ee, 0x737f, 0x3, 0x98be, 0x45, 0xf7, 0xf8fd, 0x66, 0x76fe, 0x889f, 0x3, 0x888e,
  0x15, 0xd5df, 0xa5, 0x222e, 0x98de, 0x444f, 0x3, 0xb2be, 0x85, 0xfcff, 0xf2fd, 0x226e, 0x96,
  0xb7, 0x3, 0xaaae, 0x25, 0xd1df, 0xf4fd, 0x36, 0xd4de, 0x646f, 0x3, 0xa8ae, 0x45, 0xeaef, 0xf8fd,
  0x445e, 0xe8ee, 0x717f, 0x3, 0x323e, 0x15, 0xc4cf, 0xa5, 0xfaff, 0x88ce, 0x313f, 0x3, 0xc6, 0x85,
  0x77ff, 0xf2fd, 0x647e, 0xf1fe, 0xb3bf, 0x3, 0xa2ae, 0x25, 0xe7, 0xf4fd, 0x56, 0xe2ee, 0x77, 0x3,
  0x98be, 0x45, 0xe4ef, 0xf8fd, 0x66, 0x76fe, 0x667f, 0x3, 0x888e, 0x15, 0xd7, 0xa5, 0x222e,
  0x98de, 0x333f, 0x3, 0xb2be, 0x85, 0x75ff, 0xf2fd, 0x226e, 0x96, 0x919f, 0x3, 0xaaae, 0x25,
  0x99df, 0xf4fd, 0x36, 0xd4de, 0x515f, 0x3, 0xa8ae, 0x45, 0xecef, 0xf8fd, 0x445e, 0xe8ee, 0x727f,
  0x3, 0x323e, 0x15, 0xb1bf, 0xa5, 0xf3ff, 0x88ce, 0x111f, 0x3, 0x54de, 0xf2fd, 0x111e, 0x14,
  0x647e, 0xf8fe, 0xcccf, 0x3, 0x91be, 0x45, 0x22ef, 0x25, 0x222e, 0xf3fe, 0x888f, 0x3, 0xc6, 0x85,
  0xf7, 0x14, 0x115e, 0xfcfe, 0xa8af, 0x3, 0xa6, 0x35, 0xc8df, 0xf1fd, 0x313e, 0x66fe, 0x646f, 0x3,
  0xc8ce, 0xf2fd, 0xf5ff, 0x14, 0x66, 0xf4fe, 0xbabf, 0x3, 0x22ae, 0x45, 0xe7, 0x25, 0x323e,
  0xeafe, 0x737f, 0x3, 0xb2be, 0x85, 0x55df, 0x14, 0x56, 0x717e, 0x119f, 0x3, 0x96, 0x35, 0xc4cf,
  0xf1fd, 0x333e, 0xe8ee, 0x444f, 0x3, 0x54de, 0xf2fd, 0x111e, 0x14, 0x647e, 0xf8fe, 0x99bf, 0x3,
  0x91be, 0x45, 0xe2ef, 0x25, 0x222e, 0xf3fe, 0x667f, 0x3, 0xc6, 0x85, 0xe4ef, 0x14, 0x115e,
  0xfcfe, 0x989f, 0x3, 0xa6, 0x35, 0xd7, 0xf1fd, 0x313e, 0x66fe, 0x226f, 0x3, 0xc8ce, 0xf2fd,
  0xb9ff, 0x14, 0x66, 0xf4fe, 0xb7, 0x3, 0x22ae, 0x45, 0xd1df, 0x25, 0x323e, 0xeafe, 0x77, 0x3,
  0xb2be, 0x85, 0xecef, 0x14, 0x56, 0x717e, 0x727f, 0x3, 0x96, 0x35, 0xb8bf, 0xf1fd, 0x333e,
  0xe8ee, 0x545f, 0xf1fc, 0xd1de, 0xfafd, 0xd7, 0xf8fc, 0x16, 0xfffd, 0x747f, 0xf4fc, 0x717e,
  0xf3fd, 0xb3bf, 0xf2fc, 0xeaef, 0xe8ee, 0x444f, 0xf1fc, 0x22ae, 0x5, 0xb8bf, 0xf8fc, 0xf7,
  0xfcfe, 0x77, 0xf4fc, 0x115e, 0xf5fd, 0x757f, 0xf2fc, 0xd8df, 0xe2ee, 0x333f, 0xf1fc, 0xb2be,
  0xfafd, 0x88cf, 0xf8fc, 0xfbff, 0xfffd, 0x737f, 0xf4fc, 0x6e, 0xf3fd, 0xb7, 0xf2fc, 0x66ef,
  0xf9fe, 0x313f, 0xf1fc, 0x9e, 0x5, 0xbabf, 0xf8fc, 0xfdff, 0xf6fe, 0x67, 0xf4fc, 0x26, 0xf5fd,
  0x888f, 0xf2fc, 0xdcdf, 0xd4de, 0x222f, 0xf1fc, 0xd1de, 0xfafd, 0xc4cf, 0xf8fc, 0x16, 0xfffd,
  0x727f, 0xf4fc, 0x717e, 0xf3fd, 0x99bf, 0xf2fc, 0xecef, 0xe8ee, 0x47, 0xf1fc, 0x22ae, 0x5, 0xa7,
  0xf8fc, 0xf7ff, 0xfcfe, 0x57, 0xf4fc, 0x115e, 0xf5fd, 0x97, 0xf2fc, 0xd5df, 0xe2ee, 0x37, 0xf1fc,
  0xb2be, 0xfafd, 0xc7, 0xf8fc, 0xfeff, 0xfffd, 0x667f, 0xf4fc, 0x6e, 0xf3fd, 0xa8af, 0xf2fc, 0xe7,
  0xf9fe, 0x323f, 0xf1fc, 0x9e, 0x5, 0xb1bf, 0xf8fc, 0xe4ef, 0xf6fe, 0x545f, 0xf4fc, 0x26, 0xf5fd,
  0x87, 0xf2fc, 0x99df, 0xd4de, 0x111f,
];
pub const vlc_tbl1: [OPJ_UINT16; 1024] = [
  0x13, 0x65, 0x43, 0xde, 0x83, 0x888d, 0x23, 0x444e, 0x13, 0xa5, 0x43, 0x88ae, 0x83, 0x35, 0x23,
  0xd7, 0x13, 0xc5, 0x43, 0x9e, 0x83, 0x55, 0x23, 0x222e, 0x13, 0x95, 0x43, 0x7e, 0x83, 0x10fe,
  0x23, 0x77, 0x13, 0x65, 0x43, 0x88ce, 0x83, 0x888d, 0x23, 0x111e, 0x13, 0xa5, 0x43, 0x5e, 0x83,
  0x35, 0x23, 0xe7, 0x13, 0xc5, 0x43, 0xbe, 0x83, 0x55, 0x23, 0x11ff, 0x13, 0x95, 0x43, 0x3e, 0x83,
  0x40ee, 0x23, 0xa2af, 0x13, 0x65, 0x43, 0xde, 0x83, 0x888d, 0x23, 0x444e, 0x13, 0xa5, 0x43,
  0x88ae, 0x83, 0x35, 0x23, 0x44ef, 0x13, 0xc5, 0x43, 0x9e, 0x83, 0x55, 0x23, 0x222e, 0x13, 0x95,
  0x43, 0x7e, 0x83, 0x10fe, 0x23, 0xb7, 0x13, 0x65, 0x43, 0x88ce, 0x83, 0x888d, 0x23, 0x111e, 0x13,
  0xa5, 0x43, 0x5e, 0x83, 0x35, 0x23, 0xc4cf, 0x13, 0xc5, 0x43, 0xbe, 0x83, 0x55, 0x23, 0xf7, 0x13,
  0x95, 0x43, 0x3e, 0x83, 0x40ee, 0x23, 0x6f, 0x1, 0x84, 0x1, 0x56, 0x1, 0x14, 0x1, 0xd7, 0x1,
  0x24, 0x1, 0x96, 0x1, 0x45, 0x1, 0x77, 0x1, 0x84, 0x1, 0xc6, 0x1, 0x14, 0x1, 0x888f, 0x1, 0x24,
  0x1, 0xf7, 0x1, 0x35, 0x1, 0x222f, 0x1, 0x84, 0x1, 0x40fe, 0x1, 0x14, 0x1, 0xb7, 0x1, 0x24, 0x1,
  0xbf, 0x1, 0x45, 0x1, 0x67, 0x1, 0x84, 0x1, 0xa6, 0x1, 0x14, 0x1, 0x444f, 0x1, 0x24, 0x1, 0xe7,
  0x1, 0x35, 0x1, 0x113f, 0x1, 0x84, 0x1, 0x56, 0x1, 0x14, 0x1, 0xcf, 0x1, 0x24, 0x1, 0x96, 0x1,
  0x45, 0x1, 0x6f, 0x1, 0x84, 0x1, 0xc6, 0x1, 0x14, 0x1, 0x9f, 0x1, 0x24, 0x1, 0xef, 0x1, 0x35,
  0x1, 0x323f, 0x1, 0x84, 0x1, 0x40fe, 0x1, 0x14, 0x1, 0xaf, 0x1, 0x24, 0x1, 0x44ff, 0x1, 0x45,
  0x1, 0x5f, 0x1, 0x84, 0x1, 0xa6, 0x1, 0x14, 0x1, 0x7f, 0x1, 0x24, 0x1, 0xdf, 0x1, 0x35, 0x1,
  0x111f, 0x1, 0x24, 0x1, 0x56, 0x1, 0x85, 0x1, 0xbf, 0x1, 0x14, 0x1, 0xf7, 0x1, 0xc6, 0x1, 0x77,
  0x1, 0x24, 0x1, 0xf8ff, 0x1, 0x45, 0x1, 0x7f, 0x1, 0x14, 0x1, 0xdf, 0x1, 0xa6, 0x1, 0x313f, 0x1,
  0x24, 0x1, 0x222e, 0x1, 0x85, 0x1, 0xb7, 0x1, 0x14, 0x1, 0x44ef, 0x1, 0xa2ae, 0x1, 0x67, 0x1,
  0x24, 0x1, 0x51ff, 0x1, 0x45, 0x1, 0x97, 0x1, 0x14, 0x1, 0xcf, 0x1, 0x36, 0x1, 0x223f, 0x1, 0x24,
  0x1, 0x56, 0x1, 0x85, 0x1, 0xb2bf, 0x1, 0x14, 0x1, 0x40ef, 0x1, 0xc6, 0x1, 0x6f, 0x1, 0x24, 0x1,
  0x72ff, 0x1, 0x45, 0x1, 0x9f, 0x1, 0x14, 0x1, 0xd7, 0x1, 0xa6, 0x1, 0x444f, 0x1, 0x24, 0x1,
  0x222e, 0x1, 0x85, 0x1, 0xa8af, 0x1, 0x14, 0x1, 0xe7, 0x1, 0xa2ae, 0x1, 0x5f, 0x1, 0x24, 0x1,
  0x44ff, 0x1, 0x45, 0x1, 0x888f, 0x1, 0x14, 0x1, 0xaaaf, 0x1, 0x36, 0x1, 0x111f, 0x2, 0xf8fe,
  0x24, 0x56, 0x2, 0xb6, 0x85, 0x66ff, 0x2, 0xce, 0x14, 0x111e, 0x2, 0x96, 0x35, 0xa8af, 0x2, 0xf6,
  0x24, 0x313e, 0x2, 0xa6, 0x45, 0xb3bf, 0x2, 0xb2be, 0x14, 0xf5ff, 0x2, 0x66, 0x517e, 0x545f, 0x2,
  0xf2fe, 0x24, 0x222e, 0x2, 0x22ae, 0x85, 0x44ef, 0x2, 0xc6, 0x14, 0xf4ff, 0x2, 0x76, 0x35,
  0x447f, 0x2, 0x40de, 0x24, 0x323e, 0x2, 0x9e, 0x45, 0xd7, 0x2, 0x88be, 0x14, 0xfaff, 0x2, 0x115e,
  0xf1fe, 0x444f, 0x2, 0xf8fe, 0x24, 0x56, 0x2, 0xb6, 0x85, 0xc8ef, 0x2, 0xce, 0x14, 0x111e, 0x2,
  0x96, 0x35, 0x888f, 0x2, 0xf6, 0x24, 0x313e, 0x2, 0xa6, 0x45, 0x44df, 0x2, 0xb2be, 0x14, 0xa8ff,
  0x2, 0x66, 0x517e, 0x6f, 0x2, 0xf2fe, 0x24, 0x222e, 0x2, 0x22ae, 0x85, 0xe7, 0x2, 0xc6, 0x14,
  0xe2ef, 0x2, 0x76, 0x35, 0x727f, 0x2, 0x40de, 0x24, 0x323e, 0x2, 0x9e, 0x45, 0xb1bf, 0x2, 0x88be,
  0x14, 0x73ff, 0x2, 0x115e, 0xf1fe, 0x333f, 0x1, 0x84, 0x1, 0x20ee, 0x1, 0xc5, 0x1, 0xc4cf, 0x1,
  0x44, 0x1, 0x32ff, 0x1, 0x15, 0x1, 0x888f, 0x1, 0x84, 0x1, 0x66, 0x1, 0x25, 0x1, 0xaf, 0x1, 0x44,
  0x1, 0x22ef, 0x1, 0xa6, 0x1, 0x5f, 0x1, 0x84, 0x1, 0x444e, 0x1, 0xc5, 0x1, 0xcccf, 0x1, 0x44,
  0x1, 0xf7, 0x1, 0x15, 0x1, 0x6f, 0x1, 0x84, 0x1, 0x56, 0x1, 0x25, 0x1, 0x9f, 0x1, 0x44, 0x1,
  0xdf, 0x1, 0x30fe, 0x1, 0x222f, 0x1, 0x84, 0x1, 0x20ee, 0x1, 0xc5, 0x1, 0xc8cf, 0x1, 0x44, 0x1,
  0x11ff, 0x1, 0x15, 0x1, 0x77, 0x1, 0x84, 0x1, 0x66, 0x1, 0x25, 0x1, 0x7f, 0x1, 0x44, 0x1, 0xe7,
  0x1, 0xa6, 0x1, 0x37, 0x1, 0x84, 0x1, 0x444e, 0x1, 0xc5, 0x1, 0xb7, 0x1, 0x44, 0x1, 0xbf, 0x1,
  0x15, 0x1, 0x3f, 0x1, 0x84, 0x1, 0x56, 0x1, 0x25, 0x1, 0x97, 0x1, 0x44, 0x1, 0xd7, 0x1, 0x30fe,
  0x1, 0x111f, 0x2, 0xa8ee, 0x44, 0x888e, 0x2, 0xd6, 0xc5, 0xf3ff, 0x2, 0xfcfe, 0x25, 0x3e, 0x2,
  0xb6, 0x55, 0xd8df, 0x2, 0xf8fe, 0x44, 0x66, 0x2, 0x207e, 0x85, 0x99ff, 0x2, 0xe6, 0xf5, 0x36,
  0x2, 0xa6, 0x15, 0x9f, 0x2, 0xf2fe, 0x44, 0x76, 0x2, 0x44ce, 0xc5, 0x76ff, 0x2, 0xf1fe, 0x25,
  0x444e, 0x2, 0xae, 0x55, 0xc8cf, 0x2, 0xf4fe, 0x44, 0x445e, 0x2, 0x10be, 0x85, 0xe4ef, 0x2,
  0x54de, 0xf5, 0x111e, 0x2, 0x96, 0x15, 0x222f, 0x2, 0xa8ee, 0x44, 0x888e, 0x2, 0xd6, 0xc5,
  0xfaff, 0x2, 0xfcfe, 0x25, 0x3e, 0x2, 0xb6, 0x55, 0x11bf, 0x2, 0xf8fe, 0x44, 0x66, 0x2, 0x207e,
  0x85, 0x22ef, 0x2, 0xe6, 0xf5, 0x36, 0x2, 0xa6, 0x15, 0x227f, 0x2, 0xf2fe, 0x44, 0x76, 0x2,
  0x44ce, 0xc5, 0xd5ff, 0x2, 0xf1fe, 0x25, 0x444e, 0x2, 0xae, 0x55, 0x6f, 0x2, 0xf4fe, 0x44,
  0x445e, 0x2, 0x10be, 0x85, 0x11df, 0x2, 0x54de, 0xf5, 0x111e, 0x2, 0x96, 0x15, 0x515f, 0x3, 0xf6,
  0x14, 0x111e, 0x44, 0x888e, 0xa5, 0xd4df, 0x3, 0xa2ae, 0x55, 0x76ff, 0x24, 0x223e, 0xb6, 0xaaaf,
  0x3, 0xe6, 0x14, 0xf5ff, 0x44, 0x66, 0x85, 0xcccf, 0x3, 0x9e, 0xc5, 0x44ef, 0x24, 0x36, 0xf8fe,
  0x317f, 0x3, 0xe8ee, 0x14, 0xf1ff, 0x44, 0x76, 0xa5, 0xc4cf, 0x3, 0x227e, 0x55, 0xd1df, 0x24,
  0x444e, 0xf4fe, 0x515f, 0x3, 0xd6, 0x14, 0xe2ef, 0x44, 0x445e, 0x85, 0x22bf, 0x3, 0x96, 0xc5,
  0xc8df, 0x24, 0x222e, 0xf2fe, 0x226f, 0x3, 0xf6, 0x14, 0x111e, 0x44, 0x888e, 0xa5, 0xb1bf, 0x3,
  0xa2ae, 0x55, 0x33ff, 0x24, 0x223e, 0xb6, 0xa8af, 0x3, 0xe6, 0x14, 0xb9ff, 0x44, 0x66, 0x85,
  0xa8bf, 0x3, 0x9e, 0xc5, 0xe4ef, 0x24, 0x36, 0xf8fe, 0x646f, 0x3, 0xe8ee, 0x14, 0xfcff, 0x44,
  0x76, 0xa5, 0xc8cf, 0x3, 0x227e, 0x55, 0xeaef, 0x24, 0x444e, 0xf4fe, 0x747f, 0x3, 0xd6, 0x14,
  0xfaff, 0x44, 0x445e, 0x85, 0xb2bf, 0x3, 0x96, 0xc5, 0x44df, 0x24, 0x222e, 0xf2fe, 0x313f, 0xf3,
  0xfafe, 0xf1fd, 0x36, 0x4, 0x32be, 0x75, 0x11df, 0xf3, 0x54de, 0xf2fd, 0xe4ef, 0xd5, 0x717e,
  0xfcfe, 0x737f, 0xf3, 0xf3fe, 0xf8fd, 0x111e, 0x4, 0x96, 0x55, 0xb1bf, 0xf3, 0xce, 0xb5, 0xd8df,
  0xf4fd, 0x66, 0xb9fe, 0x545f, 0xf3, 0x76fe, 0xf1fd, 0x26, 0x4, 0xa6, 0x75, 0x9f, 0xf3, 0xae,
  0xf2fd, 0xf7ff, 0xd5, 0x46, 0xf5fe, 0x747f, 0xf3, 0xe6, 0xf8fd, 0x16, 0x4, 0x86, 0x55, 0x888f,
  0xf3, 0xc6, 0xb5, 0xe2ef, 0xf4fd, 0x115e, 0xa8ee, 0x113f, 0xf3, 0xfafe, 0xf1fd, 0x36, 0x4,
  0x32be, 0x75, 0xd1df, 0xf3, 0x54de, 0xf2fd, 0xfbff, 0xd5, 0x717e, 0xfcfe, 0x447f, 0xf3, 0xf3fe,
  0xf8fd, 0x111e, 0x4, 0x96, 0x55, 0x727f, 0xf3, 0xce, 0xb5, 0x22ef, 0xf4fd, 0x66, 0xb9fe, 0x444f,
  0xf3, 0x76fe, 0xf1fd, 0x26, 0x4, 0xa6, 0x75, 0x11bf, 0xf3, 0xae, 0xf2fd, 0xffff, 0xd5, 0x46,
  0xf5fe, 0x323f, 0xf3, 0xe6, 0xf8fd, 0x16, 0x4, 0x86, 0x55, 0x6f, 0xf3, 0xc6, 0xb5, 0xb8bf,
  0xf4fd, 0x115e, 0xa8ee, 0x222f,
];
