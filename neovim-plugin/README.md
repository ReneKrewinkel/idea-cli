# Neovim user command for running `idea` command

Required file tree:

```shell
~/.config/nvim/
├── init.lua
└── lua/
    └── idea-cli/
        ├── init.lua
        └── idea-cli.lua
```

- init.lua:

```lua
require('idea-cli')
```

- idea-cli/init.lua:

````lua
require('idea-cli.idea-cli')

local idea_cli = require('idea-cli.idea-cli')
vim.api.nvim_create_user_command('IdeaCli', function()
idea_cli.run()
end, {})


- idea-cli/idea-cli.lua:

```lua
local M = {}

function M.msg(type, msg, title)
  require('notify')(msg, type, { title = title, timeout = 4000 })
end

function M.run(opts)
  opts = opts or {}
  local title = 'Idea CLI'

  local idea_string = vim.fn.input(title .. ': ')
  if not idea_string or idea_string == '' then
    return
  end

  local cmd = "idea '" .. idea_string .. "'"
  M.msg('info', 'Executing: ' .. cmd, title)

  vim.fn.jobstart(cmd, {
    on_exit = function(_, code)
      if code == 0 then
        M.msg('success', 'Idea CLI ran successfully, check the ouput in Obsidian', title)
      else
        M.msg('error', 'Idea CLI failed', title)
      end
    end
  })
end

return M
```
````
