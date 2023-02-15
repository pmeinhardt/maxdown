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
  let l:cmd = s:cmd . ' --dangerous --output ' . shellescape(a:fpath) . ' -'
  call s:exec(l:cmd, a:bnum)
endfunction

function! s:show(fpath, title) abort
  try
    call ql#view(a:fpath, a:title)
  catch /^Vim\%((\a\+)\)\=:E117:/
    if executable('qlmanage')
      call s:exec('qlmanage -p -c public.html ' . shellescape(a:fpath))
    else
      call s:exec('open ' . shellescape(a:fpath))
    endif
  endtry
endfunction

function! s:tempname(ext) abort
  let temp = tempname()
  return temp . a:ext
endfunction

function! s:preview() abort
  let temp = s:tempname('.html')
  call s:convert(temp, bufnr('%'))
  call s:show(temp, expand('%:t'))
endfunction

nnoremap <silent> <Plug>MaxdownCompile :call <SID>compile()<CR>
nnoremap <silent> <Plug>MaxdownPreview :call <SID>preview()<CR>

command! MaxdownCompile call s:compile()
command! MaxdownPreview call s:preview()
