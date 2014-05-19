#include <stdint.h>

typedef unsigned long ulong;
typedef unsigned short ushort;

struct chunk1 {
	unsigned short tag;//PCM = 1
	unsigned short numchannels;//Niet groter dan 2
	unsigned long samplerate;
	unsigned long byterate;
	unsigned short blockalign;
	unsigned short bitspersample;
};


struct Sound {
	chunk1 fmt;
	unsigned short unk1;//0
	unsigned long unk2;
	float unk3;
	float unk4;
	float unk5;
	unsigned long unk6;
	float unk7;
	unsigned long unk8;
	float unk9;
	float unk10;
	float unk11;
	long loop;
	unsigned long channel1;
	long channel1_size;
	unsigned long channel2;
	long channel2_size;
};

//Zelf
struct MorhpMeshVertex
{
	uint8_t X;
	uint8_t Y;
	uint8_t Z;
};

struct TrackSection
{
	ulong internalname;
	ulong sidref;
	ulong entrycurve;
	ulong exitcurve;
	ulong specialcurves;
	ulong direction;
	ulong entryflags;
	ulong exitflags;
	ulong splineleft;
	ulong splineright;
	ulong joinsplineleft;
	ulong joinsplineright;
	ulong extrasplineleft;
	ulong extrasplineright;
	ulong unk15;
	ulong unk16;
	ulong unk17;
	ulong unk18;
	ulong entryslope;
	ulong entrybank;
	ulong entrytrackgroup;
	ulong unk22;
	ulong unk23;
	ulong unk24;
	ulong unk245;
	ulong exitslope;
	ulong exitbank;
	ulong exittrackgroup;
};

struct SceneryItem
{
	ulong unk1;
	ulong flags;
	ushort position;
	ushort structureversion;
	ulong unk4;
	ulong squares_x;
	ulong squares_z;
	ulong siddataptr;
	float posx;
	float posy;
	float posz;
	float sizex;
	float sizey;
	float sizez;
	ulong trackstructcount;
	ulong trackstructptr;
	int cost;
	int removalcost;
	ulong unk17;
	ulong type;
	ulong supports;
	ulong svdcount;
	ulong svdref;
	ulong iconref;
	ulong groupiconref;
	ulong groupnameref;
	ulong ovlpath;
	ulong paramcount;
	ulong params;
	ulong soundcount;
	ulong sounds;
	ulong nameref;
	ulong unk27;
	ulong unk28;
	ulong stallunknown1;
	ulong stallunknown2;
	ulong col1;
	ulong col2;
	ulong col3;
	/*ulong unk34;
	ulong unk35;
	ulong unk36;
	ulong unk37;
	ulong unk38;
	ulong unk39;
	ulong unk40;
	ulong unk41;*/
};

struct SpriteCoords
{
	float left;// each coordinate can be between 0.0 - 1.0
	float top;
	float right;
	float bottom;
};

struct ParticleSkin
{
	unsigned long spritecount; // Seen between 1-16
	unsigned long pos;
	unsigned long tex_ref;
	unsigned long flaga; // Seen 0 and 1
	unsigned long flagb; // 0 = Animated, 1 = Static
	float unknownmodifier; // Seen between 0.125 and 2.0, usually 1.0
	unsigned long unk1; // Always 0
};