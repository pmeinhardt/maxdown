if exists('g:loaded_maxdown')
  finish
endif
let g:loaded_maxdown = 1

let s:profile = 'release'

let s:path = fnamemodify(fnamemodify(resolve(expand('<sfile>:p')), ':h'), ':h')
let s:cmd = s:path . '/target/' . s:profile . '/maxdown'
let s:template = s:path . '/templates/preview-template.html'

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

function! s:compile(bang) abort
  if a:bang
    call s:exec('cd ' . s:path . ' && cargo clean')
  endif

  call s:exec('cd ' . s:path . ' && cargo build --release --locked')
endfunction

function! s:convert(bang) abort
  let cmd = s:cmd

  let args = [
        \ '--title', shellescape(expand('%:t')),
        \ ]

  if a:bang
    let args += ['--dangerous']
  endif

  execute '%!' . join([cmd] + args)
endfunction

function! s:invoke(bang, dest, source, bnum) abort
  let cmd = s:cmd

  let args = [
        \ '--base', shellescape(a:source),
        \ '--output', shellescape(a:dest),
        \ '--template', shellescape(s:template),
        \ ]

  if a:bang
    let args += ['--dangerous']
  endif

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

function! s:preview(bang) abort
  let source = expand('%:p')
  let dest = expand('~/.maxdown.preview.html')
  call s:invoke(a:bang, dest, source, bufnr('%'))
  call setfperm(dest, 'rw-------')
  call s:show(dest, expand('%:t'))
endfunction

nnoremap <silent> <Plug>MaxdownCompile :call <SID>compile(0)<CR>
nnoremap <silent> <Plug>MaxdownConvert :call <SID>convert(0)<CR>
nnoremap <silent> <Plug>MaxdownPreview :call <SID>preview(0)<CR>

command! -bang MaxdownCompile call s:compile(<bang>0)
command! -bang MaxdownConvert call s:convert(<bang>0)
command! -bang MaxdownPreview call s:preview(<bang>0)
