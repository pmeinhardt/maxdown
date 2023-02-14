if exists('g:loaded_maxdown')
  finish
endif
let g:loaded_maxdown = 1

let s:profile = 'release'

let s:path = fnamemodify(fnamemodify(resolve(expand('<sfile>:p')), ':h'), ':h')
let s:cmd = s:path . '/target/' . s:profile . '/maxdown'

 function! s:exec(cmd, ...)
   if a:0 > 0
     let output = system(a:cmd, a:1)
   else
     let output = system(a:cmd)
   endif

   if v:shell_error != 0
     throw output
   endif
 endfunction

function! s:compile() abort
  call s:exec('cd ' . s:path . ' && cargo build --release --locked')
endfunction

function! s:convert(fpath, bnum) abort
  call s:exec(s:cmd . ' --dangerous --output ' . a:fpath . ' -', a:bnum)
endfunction

function! s:show(fpath) abort
  call s:exec('qlmanage -p -c public.html ' . a:fpath)
endfunction

function! s:preview() abort
  let tmp = fnameescape(tempname())
  call s:convert(tmp, bufnr('%'))
  call s:show(tmp)
endfunction

nnoremap <silent> <Plug>MaxdownCompile :call <SID>compile()<CR>
nnoremap <silent> <Plug>MaxdownPreview :call <SID>preview()<CR>

command! MaxdownCompile call s:compile()
command! MaxdownPreview call s:preview()
