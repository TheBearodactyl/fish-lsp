local util = require 'lspconfig.util'

return {
  default_config = {
    cmd = { 'fish-lsp' },
    filetypes = { 'fish' },
    root_dir = function(fname)
      return util.root_pattern '*.fish' (fname) or util.find_git_ancestor(fname)
    end,
    single_file_support = true,
  },
  docs = {
    description = [[
a language server for the `fish` scripting language
    ]],
  },
}
