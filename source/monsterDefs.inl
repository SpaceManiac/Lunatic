// all the monster types
monsterType_t monsType[NUM_MONSTERS] = {
	{"Null", 255, 255, 0,
		0, 0, 0, 0, "", 0, 0, 0, {{255}}},
	{"Bouapha", 255, 255, 0,
		11, 29, 128, 0, "graphics\\bouapha.jsp", 0, MF_WATERWALK, 0,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 255}, // attack
			{0, 0, 0, 0, 0, 0, 0, 26, 27, 28, 255}, // die
			{23, 24, 25, 24, 23, 255}, // use item
			{17, 18, 19, 20, 21, 22, 21, 20, 21, 22, 21, 20, 19, 18, 17, 255}, // bored
			{1, 3, 1, 0, 4, 6, 4, 0, 1, 3, 1, 0, 0, 0, 26, 27, 28, 255}, // watery death
		}},
	{"Bonehead", 255, 255, 0,
		8, 38, 10, 50, "graphics\\skeleton.jsp", 0, 0, AI_Bonehead,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{20, 21, 22, 23, 24, 255}, // attack
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 255}, // die
			{25, 26, 27, 28, 29, 30, 255}, // chomp/spit projectile
			{31, 32, 32, 32, 31, 255}, // point at bouapha
			{16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 255}, // resurrect=A3
		}},
	{"Scary Bat", 255, 255, 0,
		8, 27, 5, 25, "graphics\\bat.jsp", 0, MF_FLYING, AI_Bat,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 8, 7, 255}, // attack
			{17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 255}, // die
			{10, 11, 12, 12, 12, 12, 12, 11, 10, 255}, // diving attack
			{13, 14, 15, 15, 16, 255} // bounce off during dive
		}},
	{"Eensy Weensy", 255, 255, 0,
		4, 22, 1, 10, "graphics\\spider.jsp", 0, 0, AI_Spider,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 255}, // move
			{12, 13, 14, 13, 12, 255}, // attack
			{15, 16, 17, 18, 19, 18, 19, 18, 19, 20, 20, 21, 21, 255}, // die
		}},
	{"Spitter", 255, 255, 0,
		8, 21, 10, 50, "graphics\\bigspdr.jsp", 0, 0, AI_BigSpider,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 10, 11, 12, 12, 13, 13, 255}, // attack
			{14, 15, 16, 17, 18, 17, 18, 17, 18, 19, 19, 20, 20, 255}, // die
		}},
	{"Zombie", 255, 255, 0,
		11, 32, 20, 150, "graphics\\zombie.jsp", 0, 0, AI_Zombie,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 9, 10, 11, 12, 13, 13, 14, 15, 16, 17, 18, 19, 255}, // attack
			{20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 255}, // die
		}},
	{"Egg Sac", 255, 255, 0,
		20, 19, 150, 1000, "graphics\\eggsac.jsp", 0, MF_ONEFACE | MF_ENEMYWALK | MF_NOMOVE, AI_EggSac,
		{
			{0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 1, 0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 0, 255}, // attack
			{19, 20, 21, 22, 23, 24, 25, 255}, // die
		}},
	{"Mama Spider", 255, 255, 0,
		30, 30, 200, 1500, "graphics\\mamaspdr.jsp", 0, MF_NOMOVE, AI_MamaSpider,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 10, 11, 12, 12, 13, 13, 255}, // spit (attack)
			{19, 20, 21, 22, 23, 24, 25, 25, 25, 26, 27, 28, 29, 255}, // die
			{14, 15, 16, 17, 18, 255}, // bite (A1)
		}},
	{"Pygmy", 255, 255, 0,
		10, 50, 15, 100, "graphics\\pygmy.jsp", 0, 0, AI_Pygmy,
		{
			{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 2, 1, 0, 0, 0, 4, 5, 6, 5, 4, 255}, // idle
			{8, 9, 10, 9, 8, 7, 11, 12, 13, 12, 11, 7, 255}, // move
			{29, 30, 31, 31, 32, 33, 34, 35, 36, 255}, // attack
			{37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 255}, // die
			{14, 15, 16, 17, 18, 19, 20, 16, 17, 18, 19, 15, 14, 255}, // (A1) bored (spin spear)
			{21, 22, 23, 24, 25, 26, 26, 26, 27, 28, 27, 26, 27, 28, 27, 26, 255}, // (A2) bored (yawn)
			{47, 48, 49, 48, 47, 0, 47, 48, 49, 48, 47, 255}, // (A3) angry, spotted Bouapha
		}},
	{"Aquazoid", 255, 255, 0,
		15, 20, 15, 75, "graphics\\serpent.jsp", 0, MF_AQUATIC | MF_WATERWALK, AI_Serpent,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 255}, // move
			{4, 5, 6, 7, 8, 9, 10, 11, 12, 255}, // spit
			{13, 14, 15, 16, 17, 18, 19, 255}, // die
		}},
	{"Matilda", 255, 255, 0, // the head of matilda
		40, 11, 300, 1000, "graphics\\mathead.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_FACECMD | MF_NOGRAV | MF_ENEMYWALK | MF_FREEWALK, AI_MattieSkullOrHead,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 3, 2, 1, 255}, // fire
			{1, 2, 0, 3, 4, 3, 2, 0, 3, 4, 3, 0, 1, 4, 2, 4, 3, 1, 2, 255}, // die
		}},
	{"Matilda's Skull", 255, 255, 0, // the skull of matilda (head becomes this when killed)
		40, 11, 200, 1000, "graphics\\matskull.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_FACECMD | MF_NOGRAV | MF_ENEMYWALK | MF_FREEWALK, AI_MattieSkullOrHead,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 3, 2, 1, 255}, // fire
			{1, 2, 0, 3, 4, 3, 2, 0, 3, 4, 3, 0, 1, 4, 2, 4, 3, 1, 2, 255}, // die
		}},
	{"Matilda's Brain", 255, 255, 0, // the brain of matilda (skull becomes this when killed)
		40, 7, 100, 5000, "graphics\\matbrain.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_FACECMD | MF_NOGRAV | MF_ENEMYWALK | MF_FREEWALK, AI_MattieBrain,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // no form of attack
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 255}, // die
		}},
	{"Matilda's Body", 255, 255, 0, // the body of matilda
		50, 7, 1, 1, "graphics\\matbody.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_INVINCIBLE | MF_NOGRAV | MF_ENEMYWALK, AI_MattieBody,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{0, 255}, // fire
			{1, 2, 0, 3, 4, 3, 2, 0, 3, 4, 3, 0, 1, 4, 2, 4, 3, 1, 2, 255}, // die
		}},
	{"Matilda's Claw", 255, 255, 0, // the left claw (her right)
		20, 16, 1000, 1000, "graphics\\matclaw1.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_SPRITEBOX | MF_NOGRAV | MF_ENEMYWALK | MF_FREEWALK, AI_MattieClaw,
		{
			{0, 1, 2, 3, 2, 1, 255}, // idle
			{0, 255}, // move
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 255}, // claw slash
			{1, 2, 0, 3, 4, 3, 2, 0, 3, 4, 3, 0, 1, 4, 2, 4, 3, 1, 2, 255}, // die
			{4, 5, 6, 6, 6, 6, 6, 6, 6, 5, 4, 255}, // block (A1)
		}},
	{"Matilda's Claw", 255, 255, 0, // the right claw (her left)
		20, 16, 1000, 1000, "graphics\\matclaw2.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_SPRITEBOX | MF_NOGRAV | MF_ENEMYWALK | MF_FREEWALK, AI_MattieClaw,
		{
			{0, 1, 2, 3, 2, 1, 255}, // idle
			{0, 255}, // move
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 255}, // claw slash
			{1, 2, 0, 3, 4, 3, 2, 0, 3, 4, 3, 0, 1, 4, 2, 4, 3, 1, 2, 255}, // die
			{4, 5, 6, 6, 6, 6, 6, 6, 6, 5, 4, 255}, // block (A1)
		}},
	{"Matilda's Tail", 255, 255, 0, // her tail
		80, 16, 200, 1000, "graphics\\mattail.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_INVINCIBLE | MF_FACECMD | MF_NOGRAV | MF_ENEMYWALK | MF_FREEWALK, AI_MattieTail,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 1, 2, 3, 2, 1, 255}, // fire
			{1, 2, 0, 3, 2, 0, 3, 3, 0, 1, 2, 3, 1, 2, 255}, // die
		}},
	{"Ninjabread Man", 255, 255, 0,
		12, 33, 50, 300, "graphics\\ginger.jsp", 0, 0, AI_Ginger,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 6, 255}, // move
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 19, 20, 19, 20, 21, 22, 23, 255}, // triple punch
			{27, 28, 29, 30, 31, 32, 32, 32, 32, 255}, // die
			{24, 25, 26, 26, 26, 26, 26, 255}, // jumpkick (A1)
			{25, 24, 255} // unjumpkick (A2)
		}},
	{"PUMPKIN!", 255, 255, 0,
		11, 6, 5, 500, "graphics\\pumpkin.jsp", 0, 0, AI_Pumpkin,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 0, 255}, // move
			{0, 255}, // attack unused
			{0, 0, 255}, // die (just long enough to spew shrapnel)
		}},
	{"Thingie", 255, 255, 0,
		12, 44, 10, 50, "graphics\\babything.jsp", 0, 0, AI_BabyThing,
		{
			{12, 255}, // idle
			{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 255}, // move
			{20, 21, 22, 23, 24, 25, 26, 27, 255}, // attack
			{34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 255}, // die
			{13, 14, 15, 16, 17, 16, 15, 16, 17, 16, 15, 16, 17, 16, 15, 255}, // fall asleep (A1)
			{18, 19, 18, 255}, // blink (A2)
			{28, 29, 30, 29, 28, 255}, // look right (A3)
			{31, 32, 33, 32, 31, 255}, // look left (A4)
		}},
	{"Mucho Moss", 255, 255, 0,
		15, 17, 10, 25, "graphics\\moss.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_FREEWALK | MF_ENEMYWALK | MF_WALLWALK | MF_NOSHADOW, AI_Moss,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 255}, // move=grow vertically
			{4, 5, 6, 5, 4, 255}, // attack=grow horizontally
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 255}, // die
		}},
	{"Moss Grande", 255, 255, 0,
		18, 17, 100, 250, "graphics\\mossgrnd.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_ENEMYWALK, AI_MossGrande,
		{
			{0, 1, 2, 3, 4, 5, 6, 255}, // idle
			{0, 255}, // move=unused
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 255}, // attack
			{22, 23, 24, 25, 26, 27, 28, 255}, // die
		}},
	{"Magmazoid", 255, 255, 0,
		15, 20, 20, 80, "graphics\\magmazoid.jsp", 0, MF_AQUATIC | MF_WATERWALK, AI_Magmazoid,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 255}, // move
			{4, 5, 6, 7, 8, 9, 10, 10, 10, 10, 10, 10, 10, 11, 12, 255}, // attack
			{13, 14, 15, 16, 17, 18, 19, 255}, // die
		}},
	{"Shroom", 255, 255, 0,
		15, 23, 10, 50, "graphics\\shroom.jsp", 0, 0, AI_Shroom,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 10, 11, 12, 13, 14, 255}, // attack
			{15, 16, 17, 18, 19, 20, 21, 22, 255}, // die
		}},
	{"Mush the Shroom", 255, 255, 0,
		32, 27, 200, 800, "graphics\\bgshroom.jsp", 0, MF_NOMOVE, AI_Mush,
		{
			{0, 255}, // idle
			{1, 2, 2, 1, 0, 3, 4, 4, 3, 0, 255}, // move
			{5, 6, 6, 7, 8, 9, 10, 255}, // attack=cannon fire
			{21, 22, 23, 24, 25, 26, 26, 26, 26, 255}, // die
			{11, 12, 13, 12, 13, 12, 13, 12, 13, 12, 13, 12, 13, 12, 13, 12, 13, 14, 15, 16, 255}, // A1=sporegun fire
			{17, 18, 19, 20, 19, 18, 19, 20, 19, 18, 17, 255}, // A2=angry
		}},
	{"The Thing", 255, 255, 0,
		64, 11, 800, 5000, "graphics\\thething.jsp", 0, MF_NOMOVE | MF_AQUATIC | MF_WATERWALK | MF_ENEMYWALK | MF_ONEFACE, AI_TheThing,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{3, 4, 5, 6, 7, 8, 9, 10, 10, 10, 9, 8, 7, 6, 5, 4, 3, 255}, // attack=tongue
			{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255}, // die
			{1, 2, 1, 255}, // A1=blink
		}},
	{"Tentacle", 255, 255, 0,
		32, 1, 500, 100, "graphics\\thingtent.jsp", 0, MF_NOMOVE | MF_FREEWALK | MF_NOGRAV | MF_ENEMYWALK, AI_ThingTentacle,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 0, 0, 255}, // die
		}},
	{"Tentacle", 255, 255, 0, // tentacle tip
		32, 1, 500, 100, "graphics\\thingtent2.jsp", 0, MF_NOMOVE | MF_FREEWALK | MF_NOGRAV | MF_ENEMYWALK, AI_ThingTentacle,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 0, 0, 255}, // die
		}},
	{"Super Zombie", 255, 255, 0,
		36, 38, 300, 2500, "graphics\\suprzmbe.jsp", 0, MF_NOMOVE, AI_SuperZombie,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{17, 18, 19, 20, 21, 22, 23, 24, 25, 255}, // attack=punch
			{9, 10, 11, 12, 13, 14, 15, 15, 15, 15, 15, 15, 255}, // die
			{7, 8, 9, 10, 11, 11, 11, 12, 12, 13, 13, 13, 14, 14, 14, 15, 16, 16, 255}, // A1=leap
			{26, 27, 28, 29, 30, 255}, // A2=attempted grab
			{27, 31, 32, 33, 34, 35, 34, 36, 37, 36, 34, 35, 34, 36, 37, 36, 34, 35, 34, 33, 31, 27, 255} // A3= grab n' pound
		}},
	{"Happy Stick Man", 255, 255, 0,
		16, 12, 100, 1000, "graphics\\stickman.jsp", 0, MF_ONEFACE, AI_StickMan,
		{
			{0, 255}, // idle
			{1, 0, 2, 0, 1, 0, 2, 0, 1, 0, 2, 0, 255}, // move
			{10, 11, 11, 10, 10, 10, 10, 10, 10, 10, 255}, // attack=chomp
			{3, 3, 3, 3, 3, 3, 4, 4, 5, 5, 255}, // die
			{6, 7, 7, 7, 6, 0, 8, 9, 9, 9, 8, 255}, // A1=show off muscles
			{3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 3, 255}, // A2=sadness
			{1, 0, 2, 0, 1, 0, 2, 0, 1, 0, 2, 0, 0, 0, 3, 4, 5, 255}, // watery death
		}},
	{"Baby SEAL", 255, 255, 0,
		22, 20, 20, 125, "graphics\\babyseal.jsp", 0, 0, AI_BabySeal,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 0, 255}, // move
			{6, 6, 7, 8, 7, 8, 9, 10, 9, 10, 11, 12, 11, 12, 13, 13, 13, 14, 14, 14, 255}, // attack=full auto
			{15, 16, 17, 18, 19, 19, 19, 255}, // die
		}},
	{"Cryozoid", 255, 255, 0,
		15, 23, 20, 75, "graphics\\isozoid.jsp", 0, MF_NOMOVE, AI_Isozoid,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 255}, // move
			{10, 11, 12, 13, 14, 15, 16, 255}, // attack=spit snow
			{17, 18, 19, 20, 21, 22, 255}, // die
			{4, 5, 6, 7, 8, 9, 255}, // A1=rise out of the snow
			{254, 255}, // A2=idle under the ground
		}},
	{"Snowguy", 255, 255, 0,
		26, 36, 40, 200, "graphics\\snowguy.jsp", 0, 0, AI_Snowguy,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 6, 7, 0, 255}, // move
			{24, 25, 26, 27, 28, 29, 255}, // attack=slash
			{30, 31, 32, 33, 34, 35, 254, 35, 254, 35, 254, 35, 254, 254, 35, 255}, // die
			{8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 255}, // A1=throw snowball
		}},
	{"Pengulon", 255, 255, 0,
		10, 8, 5, 50, "graphics\\penguin.jsp", 0, MF_FLYING, AI_Penguin,
		{
			{0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 255}, // idle
			{0, 1, 2, 3, 4, 5, 4, 3, 2, 1, 255}, // move
			{6, 7, 7, 7, 6, 6, 3, 3, 3, 3, 3, 255}, // attack=dive
			{0, 254, 3, 254, 5, 254, 1, 254, 4, 254, 2, 254, 3, 255}, // die
		}},
	{"Zomboni", 255, 255, 0,
		48, 9, 100, 500, "graphics\\zomboni.jsp", 0, MF_NOMOVE, AI_Zomboni,
		{
			{0, 255}, // idle
			{0, 1, 2, 0, 1, 2, 255}, // move
			{0, 255}, // attack=unused
			{3, 4, 5, 5, 5, 6, 7, 8, 8, 255}, // die
		}},
	{"Sven", 255, 255, 0,
		48, 40, 500, 5000, "graphics\\yeti.jsp", 0, MF_NOMOVE, AI_Yeti,
		{
			{0, 255}, // idle
			{0, 1, 2, 1, 0, 3, 4, 3, 255}, // move
			{14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 255}, // attack=snowball
			{35, 36, 37, 38, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 255}, // die
			{28, 29, 30, 31, 32, 33, 33, 33, 34, 34, 255}, // A1=ground pound
			{5, 6, 7, 8, 9, 10, 11, 12, 13, 255}, // A2=death yodel
			{24, 25, 26, 27, 26, 25, 26, 27, 26, 25, 26, 27, 26, 25, 24, 255}, // A3=wave hello
		}},
	{"Bjorn", 1, 4, 2,
		48, 40, 500, 5000, "!36", 0, MF_NOMOVE, AI_Yeti,
		{
			{0, 255}, // idle
			{0, 1, 2, 1, 0, 3, 4, 3, 255}, // move
			{14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 255}, // attack=snowball
			{35, 36, 37, 38, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 39, 255}, // die
			{28, 29, 30, 31, 32, 33, 33, 33, 34, 34, 255}, // A1=ground pound
			{5, 6, 7, 8, 9, 10, 11, 12, 13, 255}, // A2=death yodel
			{24, 25, 26, 27, 26, 25, 26, 27, 26, 25, 26, 27, 26, 25, 24, 255}, // A3=wave hello
		}},
	{"Geozoid", 255, 255, 0,
		15, 20, 20, 75, "graphics\\geozoid.jsp", 0, MF_NOMOVE, AI_Geozoid,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 255}, // move
			{4, 5, 6, 7, 8, 9, 10, 11, 12, 255}, // attack=spit rock
			{13, 14, 15, 16, 17, 18, 19, 255}, // die
		}},
	{"Mumble", 255, 255, 0,
		15, 28, 30, 100, "graphics\\mumble.jsp", 0, MF_NOMOVE, AI_Mumble,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 0, 6, 7, 8, 9, 10, 0, 255}, // move
			{11, 12, 13, 14, 15, 16, 17, 18, 255}, // attack=head crush
			{19, 20, 21, 22, 23, 24, 25, 26, 27, 255}, // die
		}},
	{"Djinni", 255, 255, 0,
		12, 24, 10, 50, "graphics\\djinni.jsp", 0, MF_WALLWALK | MF_NOSHADOW | MF_FLYING | MF_GHOST | MF_ENEMYWALK | MF_FREEWALK, AI_Djinni,
		{
			{0, 1, 2, 3, 2, 1, 255}, // idle
			{0, 1, 2, 3, 2, 1, 255}, // move
			{12, 13, 14, 15, 16, 17, 18, 19, 255}, // attack=punch
			{20, 21, 22, 23, 255}, // die
			{4, 5, 6, 7, 8, 9, 8, 7, 6, 7, 8, 9, 8, 7, 6, 7, 8, 9, 10, 11, 255}, // A1=scare
			{23, 22, 21, 20, 255}, // A2=materialize
		}},
	{"Magic Lamp", 255, 255, 0,
		15, 10, 150, 500, "graphics\\lamp.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_ENEMYWALK, AI_MagicLamp,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 2, 3, 4, 5, 6, 7, 8, 0, 1, 255}, // attack=summon Djinni
			{1, 5, 8, 3, 7, 2, 0, 8, 254, 1, 254, 7, 254, 5, 254, 8, 254, 2, 255}, // die
		}},
	{"Cacton", 255, 255, 0,
		20, 25, 25, 100, "graphics\\cactus.jsp", 0, 0, AI_Cactus,
		{
			{0, 0, 0, 13, 14, 15, 15, 14, 13, 0, 0, 0, 16, 17, 18, 18, 17, 16, 0, 255}, // idle
			{3, 4, 5, 4, 3, 255}, // move
			{6, 7, 8, 8, 9, 10, 11, 12, 255}, // attack=shoot spines
			{19, 20, 21, 21, 22, 23, 24, 24, 254, 24, 254, 24, 254, 24, 254, 24, 255}, // die
			{1, 2, 3, 255}, // A1=begin moving
			{3, 2, 1, 255}, // A2=stop moving
		}},
	{"Roly Poly", 255, 255, 0,
		35, 11, 2000, 1000, "graphics\\roller.jsp", 0, MF_NOMOVE | MF_ONEFACE, AI_Roller,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 0, 255}, // move (roll down the screen)
			{5, 4, 3, 2, 1, 0, 255}, // attack=roll up the screen
			{0, 0, 255}, // die
			{6, 7, 8, 9, 10, 0, 255}, // A1=roll to the right
			{10, 9, 8, 7, 6, 0, 255}, // A2=roll to the left
		}},
	{"Richie Lich", 255, 255, 0,
		50, 8, 500, 1500, "graphics\\lich.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_GHOST | MF_NOSHADOW | MF_FLYING, AI_Lich,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 3, 2, 1, 255}, // attack=open mouth
			{0, 5, 6, 7, 8, 255}, // die
		}},
	{"Dust Devil", 255, 255, 0,
		50, 23, 600, 2000, "graphics\\dustdevil.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_GHOST | MF_NOSHADOW, AI_DustDevil,
		{
			{0, 1, 2, 3, 4, 5, 255}, // idle
			{0, 1, 2, 3, 4, 5, 255}, // move
			{6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 255}, // attack=swipe pitchfork
			{17, 18, 19, 20, 21, 22, 255}, // die
		}},
	{"MechaBouapha", 255, 255, 0,
		20, 24, 500, 1000, "graphics\\mechabouapha.jsp", 0, 0, AI_MechaBouapha,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 10, 11, 12, 13, 14, 255}, // attack=hammer toss
			{15, 16, 17, 18, 19, 20, 21, 22, 23, 255}, // die
			{15, 16, 17, 18, 19, 20, 21, 22, 255}, // A1=melt
			{23, 255}, // A2=ball form
			{22, 21, 20, 19, 18, 17, 16, 15, 255}, // A3=unmelt
		}},
	{"Sphinx Arm", 255, 255, 0, // arm1
		30, 10, 1000, 2000, "graphics\\sphinxarm1.jsp", 0, MF_ONEFACE | MF_NOMOVE | MF_SPRITEBOX, AI_SphinxArm,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 255}, // attack=swipe
			{0, 1, 2, 3, 2, 3, 4, 3, 2, 3, 4, 3, 254, 4, 254, 3, 254, 4, 255}, // die
		}},
	{"Sphinx Arm", 255, 255, 0, // arm2
		30, 10, 1000, 2000, "graphics\\sphinxarm2.jsp", 0, MF_ONEFACE | MF_NOMOVE | MF_SPRITEBOX, AI_SphinxArm,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 4, 5, 6, 7, 7, 8, 9, 255}, // attack=swipe
			{0, 1, 2, 3, 2, 3, 4, 3, 2, 3, 4, 3, 254, 4, 254, 3, 254, 4, 255}, // die
		}},
	{"Sphinxter", 255, 255, 0,
		80, 11, 2000, 5000, "graphics\\sphinx.jsp", 0, MF_ONEFACE | MF_NOMOVE, AI_Sphinx,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 255}, // attack=summon mummy
			{10, 10, 10, 10, 10, 10, 10, 10, 10, 255}, // die
			{4, 5, 6, 7, 7, 6, 7, 7, 7, 6, 7, 8, 9, 10, 255}, // A1=sneeze
		}},
	{"Freakazoid", 255, 255, 0,
		15, 18, 15, 75, "graphics\\freakazoid.jsp", 0, MF_AQUATIC | MF_WATERWALK, AI_Freakazoid,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 255}, // move
			{4, 5, 6, 7, 8, 9, 9, 9, 9, 10, 255}, // attack=spit spines
			{11, 12, 13, 14, 15, 16, 17, 255}, // die
		}},
	{"Centipumpkin", 255, 255, 0, // body
		14, 5, 20, 200, "graphics\\cpbody.jsp", 0, MF_ENEMYWALK, AI_CentiBody,
		{
			{0, 255}, // idle
			{1, 2, 1, 0, 3, 4, 3, 0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 255}, // die
		}},
	{"Centipumpkin", 255, 255, 0, // head
		14, 1, 40, 500, "graphics\\cphead.jsp", 0, MF_ENEMYWALK, AI_CentiHead,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 255}, // die
		}},
	{"Wacko", 255, 255, 0,
		12, 21, 20, 100, "graphics\\wacko.jsp", 0, 0, AI_Wacko,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 10, 11, 12, 11, 10, 9, 10, 11, 12, 11, 10, 9, 10, 11, 12, 13, 14, 255}, // attack=stab
			{15, 16, 17, 18, 19, 20, 254, 20, 254, 20, 254, 20, 254, 20, 255}, // die
		}},
	{"The Boiler", 255, 255, 0,
		100, 22, 1400, 2000, "graphics\\boiler.jsp", 0, MF_NOMOVE | MF_ONEFACE, AI_Boiler,
		{
			{0, 1, 2, 3, 4, 5, 6, 7, 255}, // idle
			{0, 255}, // move
			{8, 9, 10, 11, 12, 13, 14, 15, 14, 13, 12, 11, 10, 9, 8, 255}, // attack=flames
			{16, 17, 18, 19, 20, 21, 255}, // die
		}},
	{"Great Pumpkin", 255, 255, 0,
		120, 13, 1000, 4000, "graphics\\greatpk.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_ENEMYWALK, AI_GreatPumpkin,
		{
			{0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0, 0, 0, 8, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 3, 4, 5, 6, 6, 7, 8, 255}, // attack=vomit babies
			{9, 9, 10, 10, 11, 11, 12, 12, 255}, // die
		}},
	{"The Ultrazoid", 255, 255, 0,
		40, 17, 750, 2500, "graphics\\ultrazoid.jsp", 0, MF_NOMOVE | MF_ONEFACE, AI_Ultrazoid,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 4, 5, 6, 7, 8, 9, 255}, // attack=breathe
			{10, 11, 12, 13, 14, 15, 16, 255}, // die
		}},
	{"Dr. Lunatic", 255, 255, 0,
		40, 21, 50, 10000, "graphics\\dr-l.jsp", 0, 0, AI_DrLunatic,
		{
			{0, 255}, // idle
			{7, 8, 9, 8, 7, 0, 10, 11, 12, 11, 10, 0, 255}, // move
			{1, 2, 3, 4, 5, 6, 5, 4, 5, 6, 5, 6, 5, 4, 3, 4, 3, 4, 5, 4, 3, 2, 1, 255}, // attack=laugh
			{13, 14, 15, 16, 17, 17, 17, 17, 17, 17, 17, 18, 19, 20, 20, 20, 255}, // die
			// these following are only used when you play as Dr. L
			{1, 2, 3, 2, 1, 255}, // use item
			{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 255}, // bored
			{7, 9, 7, 0, 10, 12, 10, 0, 7, 9, 7, 0, 0, 0, 0, 0, 0, 255}, // watery death
		}},
	{"Super Duper Zombie", 255, 255, 0,
		40, 48, 1000, 25000, "graphics\\sdzl.jsp", 0, MF_NOMOVE | MF_ONEFACE, AI_SDZL,
		{
			{0, 255}, // idle
			{1, 2, 1, 0, 3, 4, 3, 0, 255}, // move (to the right)
			{28, 29, 30, 31, 31, 31, 31, 31, 32, 33, 34, 35, 36, 37, 38, 39, 255}, // attack=slash
			{40, 41, 42, 43, 43, 43, 43, 43, 44, 45, 46, 47, 47, 47, 47, 47, 47, 255}, // die
			{5, 6, 5, 0, 7, 8, 7, 0, 255}, // A1=move left
			{9, 10, 11, 12, 12, 12, 12, 12, 12, 13, 14, 15, 16, 16, 17, 18, 255}, // A2=breathe
			{19, 20, 21, 21, 22, 23, 24, 25, 25, 25, 25, 25, 25, 26, 27, 255}, // A3=ground pound
		}},
	{"Santa Claus", 255, 255, 0,
		18, 25, 100, 500, "graphics\\santa.jsp", 0, 0, AI_Santa,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 255}, // attack=smack
			{20, 21, 22, 23, 24, 24, 24, 254, 24, 254, 24, 254, 24, 254, 24, 255}, // die
		}},
#ifdef EXPANDO
	// EXPANSION MONSTERS
	{"Mine Cart", 255, 255, 0,
		11, 8, 100, 500, "expgraphics\\minecar.jsp", 0, MF_NOMOVE | MF_INVINCIBLE | MF_FREEWALK | MF_NOHIT, AI_MineCart,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 6, 7, 0, 255}, // move
		}},
	{"Raft", 255, 255, 0,
		8, 8, 100, 500, "expgraphics\\raft.jsp", 0, MF_NOMOVE | MF_INVINCIBLE | MF_FREEWALK | MF_NOHIT | MF_ONEFACE | MF_AQUATIC | MF_WATERWALK, AI_Raft,
		{
			{1, 2, 3, 4, 5, 6, 0, 255}, // idle
			{1, 2, 3, 4, 5, 6, 0, 255}, // move
		}},
	{"Bouapha", 255, 255, 0, // in power armor
		20, 14, 128, 0, "expgraphics\\pwrarmor.jsp", 0, 0, 0,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 9, 9, 9, 8, 7, 255}, // attack
			{10, 11, 12, 13, 255}, // die
			{7, 7, 7, 255}, // fire missiles
			{13, 12, 11, 10, 255}, // activate
		}},
	{"Vampire", 255, 255, 0,
		12, 13, 60, 500, "expgraphics\\vampire.jsp", 0, 0, AI_Vampire,
		{
			{1, 2, 1, 0, 255}, // idle
			{1, 2, 1, 0, 255}, // move
			{3, 4, 5, 6, 7, 8, 9, 255}, // attack
			{12, 11, 10, 255}, // die
		}},
	{"Coffin", 255, 255, 0,
		25, 20, 8000, 500, "expgraphics\\coffin.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_NOSHADOW, AI_Coffin,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 255}, // attack
			{17, 18, 19, 255}, // die
		}},
	{"Spooky Ghost", 255, 255, 0,
		18, 7, 10, 300, "expgraphics\\ghost.jsp", 0, MF_WALLWALK | MF_NOSHADOW | MF_FLYING | MF_GHOST | MF_ENEMYWALK | MF_FREEWALK, AI_Ghost,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 3, 3, 2, 1, 255}, // attack (scream)
			{4, 4, 5, 5, 6, 6, 255}, // die
			{4, 5, 6, 6, 5, 4, 255}, // A1=teleport
			{254, 255}, // A2=idle invisible
			{6, 5, 4, 255}, // A3=materialize
		}},
	{"Burner", 255, 255, 0,
		18, 2, 30, 300, "expgraphics\\burner.jsp", 0, MF_NOSHADOW | MF_ONEFACE | MF_NOMOVE | MF_INVINCIBLE, AI_Burner,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // attack (scream)
			{0, 255}, // die
		}},
	{"Lefty", 255, 255, 0,
		9, 28, 10, 100, "expgraphics\\lefty.jsp", 0, 0, AI_Lefty,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{14, 15, 16, 17, 18, 19, 20, 21, 255}, // attack (punch)
			{22, 23, 24, 25, 26, 27, 26, 25, 24, 26, 254, 25, 254, 27, 254, 26, 255}, // die
			{7, 8, 9, 10, 11, 12, 13, 12, 11, 10, 11, 12, 13, 12, 11, 10, 9, 8, 7, 255}, // A1=wave
		}},
	{"Pygmy Hunter", 255, 255, 0,
		10, 41, 15, 100, "expgraphics\\pygmy2.jsp", 0, 0, AI_Pygmy2,
		{
			{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 2, 1, 0, 0, 0, 4, 5, 6, 5, 4, 255}, // idle
			{8, 9, 10, 9, 8, 7, 11, 12, 13, 12, 11, 7, 255}, // move
			{20, 21, 22, 23, 24, 25, 26, 27, 28, 255}, // attack
			{32, 33, 34, 35, 36, 37, 38, 39, 40, 255}, // die
			{14, 15, 16, 15, 14, 0, 17, 18, 19, 18, 17, 0, 255}, // (A1) bored (unga dance)
			{29, 30, 31, 30, 29, 0, 29, 30, 31, 30, 29, 255}, // (A2) angry, spotted Bouapha
		}},
	{"Pygmy Shaman", 255, 255, 0,
		10, 35, 15, 100, "expgraphics\\pygmy3.jsp", 0, 0, AI_Pygmy3,
		{
			{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 2, 1, 0, 0, 0, 4, 5, 6, 5, 4, 255}, // idle
			{8, 9, 10, 9, 8, 7, 11, 12, 13, 12, 11, 7, 255}, // move
			{20, 21, 22, 22, 22, 21, 20, 255}, // attack
			{26, 27, 28, 29, 30, 31, 32, 33, 34, 255}, // die
			{14, 15, 16, 15, 14, 0, 17, 18, 19, 18, 17, 0, 255}, // (A1) bored (unga dance)
			{23, 24, 25, 24, 23, 0, 23, 24, 25, 24, 23, 255}, // (A2) angry, spotted Bouapha
		}},
	{"Pumpkinstein", 255, 255, 0,
		18, 13, 80, 1000, "expgraphics\\pkstein.jsp", 0, MF_NOMOVE, AI_Pumpkinstein,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{0, 255}, // attack
			{7, 8, 9, 10, 11, 12, 12, 12, 12, 12, 12, 12, 255}, // die
		}},
	{"Stan Glass", 255, 255, 0,
		25, 47, 150, 1000, "expgraphics\\knight.jsp", 0, MF_GLOW | MF_ONEFACE, AI_Knight,
		{
			{14, 255}, // idle
			{15, 16, 17, 18, 19, 20, 21, 22, 14, 255}, // move
			{26, 27, 28, 29, 30, 31, 32, 33, 255}, // attack
			{34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 255}, // die
			{0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 255}, // A1 = arrive
			{23, 24, 25, 25, 25, 25, 25, 25, 25, 24, 23, 255}, // A2 = block
		}},
	{"Triceroid", 255, 255, 0,
		30, 18, 60, 250, "expgraphics\\triceroid.jsp", 0, MF_NOMOVE, AI_Triceroid,
		{
			{6, 255}, // idle
			{5, 4, 3, 2, 1, 0, 1, 2, 3, 4, 5, 6, 255}, // move
			{7, 8, 9, 9, 9, 8, 7, 255}, // attack
			{10, 11, 12, 13, 14, 15, 16, 17, 255}, // die
		}},
	{"Countess", 255, 255, 0,
		25, 28, 220, 1000, "expgraphics\\countess.jsp", 0, MF_ONEFACE, AI_Countess,
		{
			{0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255}, // idle
			{0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 255}, // move
			{13, 14, 15, 16, 17, 18, 18, 18, 18, 18, 19, 255}, // attack
			{20, 21, 22, 22, 22, 22, 23, 24, 25, 26, 27, 255}, // die
			{10, 11, 12, 12, 12, 12, 11, 10, 255}, // A1=summon boneheads
			{3, 3, 3, 255}, // A2=charge up to rush
		}},
	{"Egg", 255, 255, 0,
		12, 14, 100, 200, "expgraphics\\alienegg.jsp", 0, MF_ONEFACE | MF_ENEMYWALK | MF_NOMOVE, AI_AlienEgg,
		{
			{0, 1, 2, 3, 2, 1, 255}, // idle
			{0, 255}, // move
			{4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 255}, // attack
			{4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 255}, // die
		}},
	{"Xenoid", 255, 255, 0,
		8, 10, 1, 20, "expgraphics\\babyalien.jsp", 0, 0, AI_BabyAlien,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 6, 255}, // move
			{7, 8, 9, 9, 9, 8, 7, 255}, // attack
			{0, 255}, // die
		}},
	{"Xeno Hunter", 255, 255, 0,
		13, 29, 30, 350, "expgraphics\\alien.jsp", 0, 0, AI_Alien,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{15, 16, 17, 18, 19, 20, 21, 22, 255}, // attack
			{23, 24, 25, 26, 27, 28, 255}, // die
			{7, 8, 9, 10, 11, 12, 13, 14, 255}, // A1=spit
		}},
	{"Robopumpkin", 255, 255, 0,
		26, 7, 200, 1500, "expgraphics\\robopk.jsp", 0, MF_NOMOVE, AI_Robopumpkin,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 2, 1, 255}, // attack
			{0, 0, 0, 0, 255}, // die
			{4, 5, 6, 0, 4, 5, 6, 255}, // A1=chaingun
		}},
	{"Shock Trooper", 255, 255, 0,
		40, 23, 200, 1500, "expgraphics\\shocktr.jsp", 0, MF_NOMOVE, AI_ShockTrooper,
		{
			{0, 255}, // idle
			{1, 2, 1, 0, 3, 4, 3, 0, 255}, // move
			{10, 11, 11, 11, 12, 13, 14, 14, 15, 255}, // attack
			{16, 17, 18, 19, 20, 21, 22, 22, 22, 255}, // die
			{5, 6, 7, 7, 8, 9, 255}, // A1=lob grenade
		}},
	{"MiniBot", 255, 255, 0,
		10, 6, 5, 50, "expgraphics\\robot1.jsp", 0, 0, AI_Minibot,
		{
			{0, 255}, // idle
			{1, 2, 1, 0, 255}, // move
			{0, 0, 255}, // attack
			{3, 4, 5, 5, 5, 255}, // die
		}},
	{"MeanieBot", 255, 255, 0,
		13, 10, 30, 100, "expgraphics\\robot2.jsp", 0, 0, AI_Meaniebot,
		{
			{0, 255}, // idle
			{1, 2, 1, 0, 3, 4, 3, 0, 255}, // move
			{5, 6, 6, 6, 6, 6, 5, 255}, // attack
			{7, 8, 9, 9, 9, 9, 255}, // die
		}},
	{"RoboFactory", 255, 255, 0,
		20, 28, 150, 500, "expgraphics\\robofcty.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_ENEMYWALK, AI_Robofactory,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 255}, // attack (summon)
			{0, 0, 0, 255}, // die
			{17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 0, 255}, // A1=finish the summon
		}},
	{"Turret", 255, 255, 0,
		10, 1, 80, 200, "expgraphics\\turret.jsp", 0, MF_NOMOVE, AI_Turret,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 255}, // die
		}},
	{"Doom Bunny", 255, 255, 0,
		10, 8, 30, 1000, "expgraphics\\doombnny.jsp", 0, 0, AI_Bunny,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 6, 7, 0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 0, 255}, // die
		}},
	{"KONGOR!!", 255, 255, 0,
		80, 32, 400, 2000, "expgraphics\\kongor.jsp", 0, MF_NOMOVE | MF_ONEFACE, AI_Kongor,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 255}, // move (left)
			{7, 8, 9, 10, 10, 9, 8, 7, 255}, // attack (left pound)
			{15, 16, 17, 24, 25, 26, 27, 28, 29, 30, 31, 255}, // die
			{4, 5, 6, 5, 4, 0, 255}, // A1=move (right)
			{11, 12, 13, 14, 14, 13, 12, 11, 255}, // A2=right pound
			{15, 16, 17, 18, 19, 20, 19, 18, 21, 22, 23, 22, 21, 18, 19, 20, 19, 18, 255}, // A3=chest pound
			{21, 22, 23, 22, 21, 17, 16, 15, 255}, // A4=finish chest pound
		}},
	{"Squash", 255, 255, 0,
		8, 14, 10, 100, "expgraphics\\squash.jsp", 0, 0, AI_Squash,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 10, 9, 8, 7, 255}, // attack
			{11, 12, 13, 255}, // die
		}},
	{"UltraPygmy", 255, 255, 0,
		5, 50, 5, 100, "expgraphics\\pygmy4.jsp", 0, 0, AI_UltraPygmy,
		{
			{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 2, 1, 0, 0, 0, 4, 5, 6, 5, 4, 255}, // idle
			{8, 9, 10, 9, 8, 7, 11, 12, 13, 12, 11, 7, 255}, // move
			{29, 30, 31, 31, 32, 33, 34, 35, 36, 255}, // attack
			{37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 255}, // die
			{14, 15, 16, 17, 18, 19, 20, 16, 17, 18, 19, 15, 14, 255}, // (A1) bored (spin spear)
			{21, 22, 23, 24, 25, 26, 26, 26, 27, 28, 27, 26, 27, 28, 27, 26, 255}, // (A2) bored (yawn)
			{47, 48, 49, 48, 47, 0, 47, 48, 49, 48, 47, 255}, // (A3) angry, spotted Bouapha
		}},
	{"LoonyBot 5000", 255, 255, 0,
		120, 15, 500, 3000, "expgraphics\\loonybot.jsp", 0, MF_ONEFACE | MF_NOMOVE | MF_INVINCIBLE, AI_LoonyBot,
		{
			{0, 0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 9, 8, 7, 255}, // attack
			{10, 11, 12, 13, 14, 14, 14, 14, 14, 255}, // die
		}},
	{"LoonyBot 5000", 255, 255, 0, // the core object, used to allow it to get hit when it's open
		40, 16, 500, 3000, "expgraphics\\loonygun.jsp", 0, MF_ONEFACE | MF_NOMOVE | MF_NOSHADOW | MF_FREEWALK | MF_ENEMYWALK, AI_LoonyCore,
		{
			{254, 254, 255}, // idle
			{0, 0, 255}, // move
			{0, 0, 255}, // attack
			{0, 0, 255}, // die
		}},
	{"LoonyGun", 255, 255, 0,
		40, 16, 500, 3000, "expgraphics\\loonygun.jsp", 0, MF_ONEFACE | MF_NOMOVE | MF_FACECMD | MF_NOSHADOW | MF_ENEMYWALK | MF_FREEWALK, AI_LoonyGun,
		{
			{0, 0, 255}, // idle
			{0, 0, 255}, // move
			{0, 0, 255}, // attack
			{0, 0, 255}, // die
		}},
	{"Loony Zoomer", 255, 255, 0,
		30, 1, 100, 3000, "expgraphics\\loonyshp.jsp", 0, MF_FLYING, AI_LoonyShip,
		{
			{0, 0, 255}, // idle
			{0, 0, 255}, // move
			{0, 0, 255}, // attack
			{0, 0, 0, 0, 255}, // die
		}},

	// ------------------------------------------
	// FUN PACK MONSTERS!

	{"Buddy Bunny", 2, 0, 3,
		10, 8, 30, 1000, "!83", 0, 0, AI_BuddyBunny,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 6, 7, 0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 0, 255}, // die
		}},
	{"Good Turret", 4, 1, 5,
		10, 1, 30, 200, "!82", 0, MF_NOMOVE, AI_GoodTurret,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 255}, // die
		}},
	{"Roly Poly", 255, 255, 0,
		35, 11, 2000, 1000, "!43", 0, MF_NOMOVE | MF_ONEFACE, AI_Roller,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 0, 255}, // move (roll down the screen)
			{5, 4, 3, 2, 1, 0, 255}, // attack=roll up the screen
			{0, 0, 255}, // die
			{6, 7, 8, 9, 10, 0, 255}, // A1=roll to the right
			{10, 9, 8, 7, 6, 0, 255}, // A2=roll to the left
		}},
	{"Crazy Egg", 0, 1, 3,
		12, 14, 100, 200, "!74", 0, MF_ONEFACE | MF_ENEMYWALK | MF_NOMOVE, AI_AlienEgg,
		{
			{0, 1, 2, 3, 2, 1, 255}, // idle
			{0, 255}, // move
			{4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 12, 11, 10, 9, 8, 7, 6, 5, 4, 255}, // attack
			{4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 255}, // die
		}},

	{"Matilda-X", 4, 1, 0, // the head of matilda
		40, 11, 500, 1000, "!11", 0, MF_NOMOVE | MF_ONEFACE | MF_FACECMD | MF_NOGRAV | MF_ENEMYWALK | MF_FREEWALK, AI_Mattie2SkullOrHead,
		{
			{0, 0, 255}, // idle
			{0, 0, 255}, // move
			{1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 3, 2, 1, 255}, // fire
			{1, 2, 0, 3, 4, 3, 2, 0, 3, 4, 3, 0, 1, 4, 2, 4, 3, 1, 2, 255}, // die
		}},
	{"Matilda-X Skull", 4, 1, 0, // the skull of matilda (head becomes this when killed)
		40, 11, 400, 1000, "!12", 0, MF_NOMOVE | MF_ONEFACE | MF_FACECMD | MF_NOGRAV | MF_ENEMYWALK | MF_FREEWALK, AI_Mattie2SkullOrHead,
		{
			{0, 0, 255}, // idle
			{0, 0, 255}, // move
			{1, 2, 3, 4, 4, 4, 4, 4, 4, 4, 3, 2, 1, 255}, // fire
			{1, 2, 0, 3, 4, 3, 2, 0, 3, 4, 3, 0, 1, 4, 2, 4, 3, 1, 2, 255}, // die
		}},
	{"Matilda-X Brain", 4, 1, 0, // the brain of matilda (skull becomes this when killed)
		40, 7, 100, 5000, "!13", 0, MF_NOMOVE | MF_ONEFACE | MF_FACECMD | MF_NOGRAV | MF_ENEMYWALK | MF_FREEWALK, AI_Mattie2Brain,
		{
			{0, 0, 255}, // idle
			{0, 0, 255}, // move
			{0, 0, 255}, // no form of attack
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 255}, // die
		}},
	{"Matilda-X Body", 4, 1, 0, // the body of matilda
		50, 7, 1, 1, "!14", 0, MF_NOMOVE | MF_ONEFACE | MF_INVINCIBLE | MF_NOGRAV | MF_ENEMYWALK, AI_Mattie2Body,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{0, 255}, // fire
			{1, 2, 0, 3, 4, 3, 2, 0, 3, 4, 3, 0, 1, 4, 2, 4, 3, 1, 2, 255}, // die
		}},
	{"Matilda-X Tail", 4, 1, 0, // her tail
		80, 16, 200, 1000, "!17", 0, MF_NOMOVE | MF_ONEFACE | MF_INVINCIBLE | MF_FACECMD | MF_NOGRAV | MF_ENEMYWALK | MF_FREEWALK, AI_Mattie2Tail,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 1, 2, 3, 2, 1, 255}, // fire
			{1, 2, 0, 3, 2, 0, 3, 3, 0, 1, 2, 3, 1, 2, 255}, // die
		}},

	{"Pygmy Queen", 4, 7, 3,
		10, 35, 50, 200, "!69", 0, 0, AI_PygmyQueen,
		{
			{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 2, 1, 0, 0, 0, 4, 5, 6, 5, 4, 255}, // idle
			{8, 9, 10, 9, 8, 7, 11, 12, 13, 12, 11, 7, 255}, // move
			{20, 21, 22, 22, 22, 21, 20, 255}, // attack
			{26, 27, 28, 29, 30, 31, 32, 33, 34, 255}, // die
			{14, 15, 16, 15, 14, 0, 17, 18, 19, 18, 17, 0, 255}, // (A1) bored (unga dance)
			{23, 24, 25, 24, 23, 0, 23, 24, 25, 24, 23, 255}, // (A2) angry, spotted Bouapha
		}},
	{"Jalapeno", 1, 4, 4,
		8, 14, 5, 100, "!85", 0, 0, AI_Jalapeno,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 10, 9, 8, 7, 255}, // attack
			{11, 12, 13, 255}, // die
		}},
	{"Generator", 255, 255, 0, // once every second
		30, 10, 100, 1000, "expgraphics\\genrtr.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_FACECMD | MF_ENEMYWALK, AI_Generator,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 0, 0, 255}, // die
		}},
	{"Generator", 1, 6, 0, // once every 5 seconds
		30, 10, 200, 1000, "expgraphics\\genrtr.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_FACECMD | MF_ENEMYWALK, AI_Generator,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 0, 0, 255}, // die
		}},
	{"Generator", 1, 3, 0, // once every 15 seconds
		30, 10, 400, 1000, "expgraphics\\genrtr.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_FACECMD | MF_ENEMYWALK, AI_Generator,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 0, 0, 255}, // die
		}},
	{"Generator", 1, 5, 0, // once every 30 seconds
		30, 10, 800, 1000, "expgraphics\\genrtr.jsp", 0, MF_NOMOVE | MF_ONEFACE | MF_FACECMD | MF_ENEMYWALK, AI_Generator,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 0, 0, 255}, // die
		}},
	{"Sneaky Shark", 255, 255, 0,
		15, 13, 50, 100, "expgraphics\\shark.jsp", 0, MF_ONEFACE | MF_AQUATIC | MF_WATERWALK, AI_Shark,
		{
			{0, 1, 2, 3, 4, 5, 6, 255}, // idle
			{7, 255}, // move
			{7, 8, 9, 9, 9, 9, 9, 9, 8, 7, 255}, // attack
			{10, 11, 12, 255}, // die
			{7, 8, 9, 9, 8, 7, 255}, // A1=surprise!!!
		}},
	{"Mad Millennium Bug", 3, 4, 8,
		4, 22, 100, 500, "!4", 0, 0, AI_MadBug,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 0, 255}, // move
			{12, 13, 14, 13, 12, 255}, // attack
			{15, 16, 17, 18, 19, 18, 19, 18, 19, 20, 20, 21, 21, 255}, // die
			{12, 13, 14, 13, 12, 255}, // A1=shoot
		}},
	{"Wacky Wizard", 4, 3, 0,
		18, 25, 200, 500, "!59", 0, 0, AI_Wizard,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 255}, // attack=smack
			{20, 21, 22, 23, 24, 24, 24, 254, 24, 254, 24, 254, 24, 254, 24, 255}, // die
			{7, 9, 11, 12, 12, 12, 12, 12, 12, 11, 9, 7, 255}, // A1=cast spell
		}},
	{"Evil Clone", 1, 3, 0,
		11, 29, 128, 0, "!1", 0, 0, AI_EvilClone,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 255}, // attack
			{0, 0, 0, 0, 0, 0, 0, 26, 27, 28, 255}, // die
			{23, 24, 25, 24, 23, 255}, // use item
			{17, 18, 19, 20, 21, 22, 21, 20, 21, 22, 21, 20, 19, 18, 17, 255}, // bored
			{1, 3, 1, 0, 4, 6, 4, 0, 1, 3, 1, 0, 0, 0, 26, 27, 28, 255}, // watery death
		}},
	{"Bob The Biscuit", 255, 255, -7,
		12, 33, 70, 300, "!18", 0, 0, AI_Bob,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 6, 255}, // move
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 19, 20, 19, 20, 21, 22, 23, 255}, // triple punch
			{27, 28, 29, 30, 31, 32, 32, 32, 32, 255}, // die
			{24, 25, 26, 26, 26, 26, 26, 255}, // jumpkick (A1)
			{25, 24, 255}, // unjumpkick (A2)
			{23, 22, 21, 20, 19, 20, 19, 20, 21, 22, 23, 255}, // summon help (A3)
		}},
	{"MultiMoss", 1, 6, 0,
		18, 17, 10, 250, "!22", 0, MF_ONEFACE | MF_ENEMYWALK | MF_FREEWALK | MF_NOSHADOW, AI_MultiMoss,
		{
			{0, 1, 2, 3, 4, 5, 6, 255}, // idle
			{0, 1, 2, 3, 4, 5, 6, 255}, // move
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 255}, // attack
			{22, 23, 24, 25, 26, 27, 28, 255}, // die
			{28, 27, 26, 25, 24, 23, 22, 255}, // A1=reborn!
		}},
	{"Moss Rapido", 1, 5, 0,
		15, 17, 1, 25, "!21", 0, MF_NOMOVE | MF_ONEFACE | MF_FREEWALK | MF_ENEMYWALK | MF_WALLWALK | MF_NOSHADOW, AI_MossRapido,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 255}, // move=grow vertically
			{4, 5, 6, 5, 4, 255}, // attack=grow horizontally
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 255}, // die
		}},
	{"Snowball", 2, 0, 10,
		35, 11, 2000, 1000, "!43", 0, MF_NOMOVE | MF_ONEFACE | MF_WATERWALK, AI_Snowball,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 0, 255}, // move (roll down the screen)
			{5, 4, 3, 2, 1, 0, 255}, // attack=roll up the screen
			{0, 0, 255}, // die
			{6, 7, 8, 9, 10, 0, 255}, // A1=roll to the right
			{10, 9, 8, 7, 6, 0, 255}, // A2=roll to the left
		}},
	{"Snowball", 2, 0, 10,
		35, 11, 2000, 1000, "!43", 0, MF_NOMOVE | MF_ONEFACE | MF_WATERWALK, AI_Snowball,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 0, 255}, // move (roll down the screen)
			{5, 4, 3, 2, 1, 0, 255}, // attack=roll up the screen
			{0, 0, 255}, // die
			{6, 7, 8, 9, 10, 0, 255}, // A1=roll to the right
			{10, 9, 8, 7, 6, 0, 255}, // A2=roll to the left
		}},
	{"Snow Blower", 255, 255, 10,
		18, 2, 30, 300, "!66", 0, MF_NOSHADOW | MF_ONEFACE | MF_NOMOVE | MF_INVINCIBLE, AI_Snowblower,
		{
			{0, 255}, // idle
			{0, 255}, // move
			{0, 255}, // attack
			{0, 255}, // die
		}},
	{"Boomkin", 2, 4, 5,
		11, 6, 1, 500, "!19", 0, 0, AI_Pumpkin,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 0, 255}, // move
			{0, 255}, // attack unused
			{0, 0, 255}, // die (just long enough to spew shrapnel)
		}},
	{"Manic Mumble", 0, 7, 4,
		15, 28, 20, 100, "!39", 0, MF_NOMOVE, AI_Mumble,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 0, 6, 7, 8, 9, 10, 0, 255}, // move
			{11, 12, 13, 14, 15, 16, 17, 18, 255}, // attack=head crush
			{19, 20, 21, 22, 23, 24, 25, 26, 27, 255}, // die
		}},
	{"BuddyBot", 0, 7, -2,
		10, 6, 5, 50, "!79", 0, 0, AI_Minibot,
		{
			{0, 255}, // idle
			{1, 2, 1, 0, 255}, // move
			{0, 0, 255}, // attack
			{3, 4, 5, 5, 5, 255}, // die
		}},
	{"HelpyBot", 0, 7, -2,
		13, 10, 30, 100, "!80", 0, 0, AI_Meaniebot,
		{
			{0, 255}, // idle
			{1, 2, 1, 0, 3, 4, 3, 0, 255}, // move
			{5, 6, 6, 6, 6, 6, 5, 255}, // attack
			{7, 8, 9, 9, 9, 9, 255}, // die
		}},
	{"Xeno Queen", 0, 6, 0,
		13, 29, 200, 350, "!76", 0, MF_ENEMYWALK, AI_XenoMama,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{15, 16, 17, 18, 19, 20, 21, 22, 255}, // attack
			{23, 24, 25, 26, 27, 28, 255}, // die
			{7, 8, 9, 10, 11, 12, 13, 14, 255}, // A1=spit
		}},
	{"Rumbly Tumbly", 255, 255, 4,
		35, 11, 50, 100, "!43", 0, MF_NOMOVE | MF_ONEFACE | MF_WATERWALK, AI_Roller,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 0, 255}, // move (roll down the screen)
			{5, 4, 3, 2, 1, 0, 255}, // attack=roll up the screen
			{0, 0, 255}, // die
			{6, 7, 8, 9, 10, 0, 255}, // A1=roll to the right
			{10, 9, 8, 7, 6, 0, 255}, // A2=roll to the left
		}},
	{"Rumbly Tumbly", 255, 255, 4,
		35, 11, 50, 100, "!43", 0, MF_NOMOVE | MF_ONEFACE | MF_WATERWALK, AI_Roller,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 0, 255}, // move (roll down the screen)
			{5, 4, 3, 2, 1, 0, 255}, // attack=roll up the screen
			{0, 0, 255}, // die
			{6, 7, 8, 9, 10, 0, 255}, // A1=roll to the right
			{10, 9, 8, 7, 6, 0, 255}, // A2=roll to the left
		}},
	{"Dark Vampire", 6, 0, -6,
		12, 13, 150, 500, "!63", 0, MF_INVINCIBLE, AI_Vampire,
		{
			{1, 2, 1, 0, 255}, // idle
			{1, 2, 1, 0, 255}, // move
			{3, 4, 5, 6, 7, 8, 9, 255}, // attack
			{12, 11, 10, 255}, // die
		}},
	{"Grabby Gnome", 2, 1, 0,
		5, 50, 15, 100, "!86", 0, 0, AI_Gnome,
		{
			{0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 2, 3, 2, 1, 0, 0, 0, 4, 5, 6, 5, 4, 255}, // idle
			{8, 9, 10, 9, 8, 7, 11, 12, 13, 12, 11, 7, 255}, // move
			{29, 30, 31, 31, 32, 33, 34, 35, 36, 255}, // attack
			{37, 38, 39, 40, 41, 42, 43, 44, 45, 46, 255}, // die
			{14, 15, 16, 17, 18, 19, 20, 16, 17, 18, 19, 15, 14, 255}, // (A1) bored (spin spear)
			{21, 22, 23, 24, 25, 26, 26, 26, 27, 28, 27, 26, 27, 28, 27, 26, 255}, // (A2) bored (yawn)
			{47, 48, 49, 48, 47, 0, 47, 48, 49, 48, 47, 255}, // (A3) angry, spotted Bouapha
		}},
	{"Nobody", 255, 255, 0,
		1, 1, 1, 1, "!1", 0, MF_INVINCIBLE | MF_NOHIT | MF_FREEWALK | MF_ENEMYWALK, AI_Nobody,
		{
			{254, 255}, // idle
			{254, 255}, // move
			{254, 255}, // attack
			{254, 255}, // die
			{254, 255}, // (A1) bored (spin spear)
			{254, 255}, // (A2) bored (yawn)
			{254, 255}, // (A3) angry, spotted Bouapha
		}},
	{"Buddy Bunny", 2, 0, 3,
		10, 8, 30, 1000, "!83", 0, MF_NOMOVE, AI_BuddyBunny,
		{
			{0, 255}, // idle
			{1, 2, 3, 4, 5, 6, 7, 0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 0, 255}, // die
		}},
	{"Rock Trooper", 6, 2, 0,
		40, 23, 100, 1000, "!78", 0, MF_NOMOVE, AI_RockTrooper,
		{
			{0, 255}, // idle
			{1, 2, 1, 0, 3, 4, 3, 0, 255}, // move
			{10, 11, 11, 11, 12, 13, 14, 14, 15, 255}, // attack
			{16, 17, 18, 19, 20, 21, 22, 22, 22, 255}, // die
			{5, 6, 7, 7, 8, 9, 255}, // A1=lob grenade
		}},
	{"Jacko", 255, 255, -2, // big pumpkin
		14, 1, 30, 500, "!52", 0, 0, AI_BigPumpkin,
		{
			{0, 255}, // idle
			{0, 0, 0, 255}, // move
			{0, 255}, // attack
			{0, 0, 0, 255}, // die
		}},
	{"Crazybone", 0, 3, 0,
		8, 38, 10, 100, "!2", 0, 0, AI_CrazyBone,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 4, 5, 6, 5, 4, 0, 255}, // move
			{20, 21, 22, 23, 24, 255}, // attack
			{7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 255}, // die
			{25, 26, 27, 28, 29, 30, 255}, // chomp/spit projectile
			{31, 32, 32, 32, 31, 255}, // point at bouapha
			{16, 15, 14, 13, 12, 11, 10, 9, 8, 7, 255}, // resurrect=A3
		}},
	{"Creepazoid", 255, 255, 10,
		15, 20, 15, 75, "!10", 0, MF_WATERWALK | MF_WALLWALK | MF_FREEWALK | MF_ENEMYWALK | MF_NOSHADOW | MF_GLOW | MF_FLYING, AI_Creepazoid,
		{
			{0, 255}, // idle
			{1, 2, 3, 2, 1, 0, 255}, // move
			{4, 5, 6, 7, 8, 9, 10, 11, 12, 255}, // spit
			{13, 14, 15, 16, 17, 18, 19, 255}, // die
		}},
#endif
};
