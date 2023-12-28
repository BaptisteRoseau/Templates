--[[
  If you don't know anything about Lua, I recommend taking some time to read through
  a guide. One possible example:
  - https://learnxinyminutes.com/docs/lua/

  And then you can explore or search through `:help lua-guide`
  - https://neovim.io/doc/user/lua-guide.html
]]--

-- Leader should be defined before every plugin are initialized
vim.g.mapleader = ' '
vim.g.maplocalleader = ' '

require('baptiste')

-- The line beneath this is called `modeline`. See `:help modeline`
-- vim: ts=4 sts=4 sw=4
