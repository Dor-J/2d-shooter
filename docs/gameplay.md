# Gameplay rules, first playable release

The source game is a reference for pace and presentation, not a byte-for-byte ruleset. This implementation uses a 60 Hz authoritative tick and a 1,200 × 700 world. A player moves at up to 270 units/s, jumps with an initial upward velocity of 390 units/s, and can jet upward while fuel remains. Fuel refills on the ground. Deathmatch and team deathmatch use the same physics. Friendly fire is disabled. Players respawn after two seconds.

The initial arena has five rectangular platforms and three weapons: rifle (22 damage, 9 tick cooldown), heavy rifle (34 damage, 18 tick cooldown), and rapid rifle (12 damage, 4 tick cooldown). These values are temporary tuning targets. Further maps, weapons, and modes require content and play testing before production release.

