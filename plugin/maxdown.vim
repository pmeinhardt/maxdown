if exists('g:loaded_maxdown')
  finish
endif
let g:loaded_maxdown = 1

let s:profile = 'release'

let s:path = fnamemodify(fnamemodify(resolve(expand('<sfile>:p')), ':h'), ':h')
let s:cmd = s:path . '/target/' . s:profile . '/maxdown'

function! s:compile()
  call system('cd ' . s:path . ' && cargo build --release --locked')
endfunction

function! s:convert(fpath, bnum)
  call system(s:cmd . ' --dangerous - > ' . a:fpath, a:bnum)
endfunction

function! s:show(fpath)
  call system('qlmanage -p -c public.html ' . a:fpath)
endfunction

function! s:preview()
  let tmp = tempname()
  call s:convert(tmp, bufnr('%'))
  call s:show(tmp)
endfunction

nnoremap <silent> <Plug>MaxdownCompile :call <SID>compile()<CR>
nnoremap <silent> <Plug>MaxdownPreview :call <SID>preview()<CR>

command! MaxdownCompile call s:compile()
command! MaxdownPreview call s:preview()
