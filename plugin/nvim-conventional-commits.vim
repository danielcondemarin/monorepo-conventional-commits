" Initialize the channel
if !exists('s:conventionalCommitsJobId')
	let s:conventionalCommitsJobId = 0
endif

" The path to the binary that was created out of 'cargo build' or 'cargo build --release". This will generally be 'target/release/name'
let s:scriptdir = resolve(expand('<sfile>:p:h') . '/..')
let s:bin = s:scriptdir . '/target/release/nvim-conventional-commits'

" Entry point. Initialize RPC. If it succeeds, then attach commands to the `rpcnotify` invocations.
function! s:connect()
  let id = s:initRpc()
  
  if 0 == id
    echoerr "conventionalCommits: cannot start rpc process"
  elseif -1 == id
    echoerr "conventionalCommits: rpc process is not executable"
  else
    " Mutate our jobId variable to hold the channel ID
    let s:conventionalCommitsJobId = id 
 
    call s:configureCommands()
  endif
endfunction

function! s:suggestCommit()
  call rpcnotify(s:conventionalCommitsJobId, 'suggestCommit', getcwd())
endfunction

" Initialize RPC
function! s:initRpc()
  if s:conventionalCommitsJobId == 0
    let jobid = jobstart([s:bin], { 'rpc': v:true })
    return jobid
  else
    return s:conventionalCommitsJobId
  endif
endfunction

function! s:configureCommands()
  command! -nargs=0 SuggestCommit :call s:suggestCommit()
endfunction

call s:connect()
