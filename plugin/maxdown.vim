if exists('g:loaded_maxdown')
  finish
endif
let g:loaded_maxdown = 1

if !exists('g:maxdown_command')
  let g:maxdown_command = 'maxdown --dangerous -'
endif

if !exists('g:maxdown_preview_command')
  let g:maxdown_preview_command = 'qlmanage -p -c public.html'
endif

function! s:preview()
  let tmpfile = tempname()
  let cmd = g:maxdown_command . ' > ' . tmpfile
  call system(cmd, bufnr('%'))
  call system(g:maxdown_preview_command . ' ' . tmpfile)
endfunction

command! MaxdownPreview call s:preview()
