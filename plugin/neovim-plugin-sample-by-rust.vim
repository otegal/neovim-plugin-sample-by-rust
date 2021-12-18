" Initialize the channel
if !exists('s:rustPluginJobId')
  let s:rustPluginJobId = 0
endif

" The path to the binary
" It was created out of `cargo build --release`
let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
let s:bin = s:scriptdir . '/target/release/neovim-plugin-sample-by-rust'

" Constants for RPC messages
let s:Add = 'add'
let s:Sub = 'sub'

" Entry point
"   - Initialize RPC
"   - If it succeeds, then attach commands to the `rpcnotify`
function! s:connect()
  let id = s:initRpc()

  if 0 == id
    echoerr "cannot start rpc process"
  elseif -1 == id
    echoerr "rpc process is not executable"
  else
    let s:rustPluginJobId = id
    call s:configureCommands()
  endif
endfunction

function! s:initRpc()
  if s:rustPluginJobId == 0
    let jobId = jobstart([s:bin], { 'rpc': v:true })
    return jobId
  else
    return s:rustPluginJobId
  endif
endfunction

function! s:configureCommands()
  command! -nargs=+ Add :call s:add(<f-args>)
  command! -nargs=+ Sub :call s:sub(<f-args>)
endfunction

function! s:add(...)
  let s:p = get(a:, 1, 0)
  let s:q = get(a:, 2, 0)

  call rpcnotify(s:rustPluginJobId, s:Add, str2nr(s:p), str2nr(s:q))
endfunction

function! s:sub(...)
  let s:p = get(a:, 1, 0)
  let s:q = get(a:, 2, 0)

  call rpcnotify(s:rustPluginJobId, s:Sub, str2nr(s:p), str2nr(s:q))
endfunction

call s:connect()

