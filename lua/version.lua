local ref = Game.main_menu
function Game.main_menu(a, b)
	ref(a, b)
	UIBox:init({
		definition = {
			n = G.UIT.ROOT,
			config = {
				align = "cm",
				colour = G.C.UI.TRANSPARENT_DARK,
			},
			nodes = {
				{
					n = G.UIT.T,
					config = {
						scale = 0.3,
						text = "balatro.vs 1.0.0",
						colour = G.C.UI.TEXT_LIGHT,
					},
				},
			},
		},
		config = {
			align = "b",
			bond = "Weak",
			offset = {
				x = 0,
				y = 0,
			},
			major = G.ROOM_ATTACH,
		},
	})
end
