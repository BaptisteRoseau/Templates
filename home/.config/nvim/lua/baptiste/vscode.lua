local c = require('vscode.colors').get_colors()
require('vscode').setup({
    -- Alternatively set style in setup
    style = 'dark',

    -- Transparent background
    transparent = false,

    -- Enable italic comment
    italic_comments = true,

    -- Disable nvim-tree background color
    disable_nvimtree_bg = true,

    -- Override colors (see ./lua/vscode/colors.lua)
    color_overrides = {
        -- vscFront = '#3D3D3D'
    },

    -- Override highlight groups (see ./lua/vscode/theme.lua)
    group_overrides = {
        -- this supports the same val table as vim.api.nvim_set_hl
        -- use colors from this colorscheme by requiring vscode.colors!
        -- See https://github.com/Mofiqul/vscode.nvim/blob/main/lua/vscode/theme.lua
        -- IndentBlanklineContextChar        = { fg = "#FFFFFF", bg = 'NONE', nocombine = true },
        -- IndentBlanklineContextStart       = { fg = "#FFFFFF", bg = 'NONE', nocombine = true, underline = true },
        -- IndentBlanklineChar               = { fg = "#FFFFFF", bg = 'NONE', nocombine = true },
        -- IndentBlanklineSpaceChar          = { fg = "#FFFFFF", bg = 'NONE', nocombine = true },
        -- IndentBlanklineSpaceCharBlankline = { fg = "#FFFFFF", bg = 'NONE', nocombine = true },
        -- NvimTreeIndentMarker              = { fg = "#FFFFFF", bg = 'NONE' }
    }
})
require('vscode').load()

local highlight = {
    "CursorColumn",
    "Whitespace",

}
require('ibl').setup({
    whitespace = {
        highlight = highlight,
        remove_blankline_trail = false
    },
    scope = { enabled = false },
})
