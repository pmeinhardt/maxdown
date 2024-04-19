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

function! s:convert() abort
  let cmd = s:cmd

  let args = [
        \ '--dangerous',
        \ '--template', shellescape(s:path . '/src/minimal-template.html'),
        \ '--title', shellescape(expand('%:t')),
        \ '-',
        \ ]

  execute '%!' . join([cmd] + args)
endfunction

function! s:invoke(dest, source, bnum) abort
  let cmd = s:cmd

  let args = [
        \ '--dangerous',
        \ '--base', shellescape(a:source),
        \ '--output', shellescape(a:dest),
        \ '-'
        \ ]

  call s:exec(join([cmd] + args), a:bnum)
endfunction

function! s:show(fpath, title) abort
  try
    call ql#view(a:fpath, a:title)
  catch /^Vim\%((\a\+)\)\=:E117:/
    if executable('qlmanage')
      call s:exec('qlmanage -p -c public.html ' . shellescape(a:fpath))
    elseif executable('open')
      call s:exec('open ' . shellescape(a:fpath))
    else
      call s:exec('xdg-open ' . shellescape(a:fpath))
    endif
  endtry
endfunction

function! s:preview() abort
  let source = expand('%:p')
  let dest = expand('~/.maxdown.preview.html')
  call s:invoke(dest, source, bufnr('%'))
  call s:show(dest, expand('%:t'))
endfunction

nnoremap <silent> <Plug>MaxdownCompile :call <SID>compile()<CR>
nnoremap <silent> <Plug>MaxdownConvert :call <SID>convert()<CR>
nnoremap <silent> <Plug>MaxdownPreview :call <SID>preview()<CR>

command! MaxdownCompile call s:compile()
command! MaxdownConvert call s:convert()
command! MaxdownPreview call s:preview()
