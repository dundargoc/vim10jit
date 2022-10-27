local fn = {}

fn.insert = function(list, item, idx)
  if idx == nil then
    idx = 1
  end

  table.insert(list, idx + 1, item)

  return list
end

fn.extend = function(left, right, expr3)
  if expr3 ~= nil then
    error "haven't written this code yet"
  end

  if vim.tbl_islist(right) then
    vim.list_extend(left, right)
    return left
  else
    -- local result = vim.tbl_extend(left, right)
    for k, v in pairs(right) do
      left[k] = v
    end

    return left
  end
end

fn.add = function(list, item)
  table.insert(list, item)
  return list
end

fn.has_key = function(obj, key)
  return not not obj[key]
end

fn.prop_type_add = function(...)
  local args = { ... }
  print("[prop_type_add]", vim.inspect(args))
end

do
  local patch_overrides = {
    -- We do have vim9script :) that's this plugin
    ["vim9script"] = true,

    -- Include some vim patches that I don't care about
    [ [[patch-8.2.2261]] ] = true,
    [ [[patch-8.2.4257]] ] = true,
  }

  fn.has = function(patch)
    if patch_overrides[patch] then
      return true
    end

    return vim.fn.has(patch)
  end
end

--[=[


readdirex({directory} [, {expr} [, {dict}]])			*readdirex()*
		Extended version of |readdir()|.
		Return a list of Dictionaries with file and directory
		information in {directory}.
		This is useful if you want to get the attributes of file and
		directory at the same time as getting a list of a directory.
		This is much faster than calling |readdir()| then calling
		|getfperm()|, |getfsize()|, |getftime()| and |getftype()| for
		each file and directory especially on MS-Windows.
		The list will by default be sorted by name (case sensitive),
		the sorting can be changed by using the optional {dict}
		argument, see |readdir()|.

		The Dictionary for file and directory information has the
		following items:
			group	Group name of the entry. (Only on Unix)
			name	Name of the entry.
			perm	Permissions of the entry. See |getfperm()|.
			size	Size of the entry. See |getfsize()|.
			time	Timestamp of the entry. See |getftime()|.
			type	Type of the entry.
				On Unix, almost same as |getftype()| except:
				    Symlink to a dir	"linkd"
				    Other symlink	"link"
				On MS-Windows:
				    Normal file		"file"
				    Directory		"dir"
				    Junction		"junction"
				    Symlink to a dir	"linkd"
				    Other symlink	"link"
				    Other reparse point	"reparse"
			user	User name of the entry's owner. (Only on Unix)
		On Unix, if the entry is a symlink, the Dictionary includes
		the information of the target (except the "type" item).
		On MS-Windows, it includes the information of the symlink
		itself because of performance reasons.
--]=]
fn.readdirex = function(dir)
  print("DIR:", vim.inspect(dir))
  local files = vim.fn.readdir(dir)
  local direx = {}
  for _, f in ipairs(files) do
    table.insert(direx, {
      name = f,
      type = vim.fn.getftype(f),
    })
  end

  return direx
end

fn.mapnew = function(tbl, expr)
  return vim.fn.map(tbl, expr)
end

-- Popup menu stuff

do
  local pos_map = {
    topleft = "NW",
    topright = "NE",
    botleft = "SW",
    botright = "SE",
  }

  fn.popup_menu = function(what, options)
    -- print "OPTIONS:"
    -- P(options)

    local buf = vim.api.nvim_create_buf(false, true)
    local win = vim.api.nvim_open_win(buf, true, {
      relative = "editor",
      style = "minimal",
      anchor = pos_map[options.pos],
      height = options.maxheight or options.minheight,
      width = options.maxwidth or options.minwidth,
      row = options.line,
      col = options.col,
    })

    if options.filter then
      local loop
      loop = function()
        vim.cmd [[redraw!]]
        local ok, ch = pcall(fn.getcharstr)
        if not ok then
          return
        end -- interrupted

        if ch == "<C-C>" then
          return
        end

        if not require("vim9script").bool(options.filter(nil, ch)) then
          vim.cmd.normal(ch)
        end

        vim.schedule(loop)
      end

      vim.schedule(loop)
    end

    return win
  end
end

fn.popup_settext = function(id, text)
  if type(text) == "string" then
    -- text = vim.split(text, "\n")
    error "Haven't handled string yet"
  end

  local lines = {}
  for _, obj in ipairs(text) do
    table.insert(lines, obj.text)
  end

  vim.api.nvim_buf_set_lines(vim.api.nvim_win_get_buf(id), 0, -1, false, lines)
end

fn.popup_filter_menu = function()
  print "ok, just pretend we filtered the menu"
end

fn.popup_setoptions = function(id, options)
  print("setting options...", id)
end

fn.job_start = function(...)
  return vim.fn["vim9#job#start"](...)
end

fn.job_status = function()
  -- LOL
  return "run"
end

fn = setmetatable(fn, {
  __index = vim.fn,
})

return fn
