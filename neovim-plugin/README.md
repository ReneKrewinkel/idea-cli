# Neovim user command for running `idea` command

<img src='./neovim-plugin.png'>

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

```lua
local idea_cli = require('idea-cli.idea-cli')

vim.api.nvim_create_user_command('IdeaCli', function()
    idea_cli.run()
end, {})
```

- idea-cli/idea-cli.lua:

```lua
local M = {}

function M.run(opts)
  opts = opts or {}
  local title = 'Idea CLI'

  local idea_string = vim.fn.input(title .. ': ')
  if not idea_string or idea_string == '' then
    return
  end

  local cmd = "idea '" .. idea_string .. "'"
  vim.notify('Executing: ' .. cmd, 'info', { title = title })

  vim.fn.jobstart(cmd, {
    on_exit = function(result, code)
      if code == 0 then
      -- TODO: find a way to catch the file name it created and 
      -- open this in a new buffer
        -- vim.cmd('e' .. result)
        vim.notify('Check result in Obsidian', 'succes', { title = title })
      else
        vim.notify('Idea CLI failed', 'error', { title = title })
      end
    end
  })
end

return M

```

