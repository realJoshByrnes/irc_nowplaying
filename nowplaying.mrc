;*** FOR DEBUGGING ***
on 1:START:{
  echo -at Loaded: $dll($qt($$cmdline), version, $null)
  noop $irc_nowplaying(wait_for_media, data).irc_nowplaying:mediachanged
}

; Overrides (testing)
alias -l irc_nowplaying.dll return $cmdline

; Usage:
;    Alias (Sync): $irc_nowplaying(procname, data)
;   Alias (Async): $irc_nowplaying(procname, data).callback
; Command (Async): /irc_nowplaying procname data
;  Command (Sync): /irc_nowplaying -s procname data

alias irc_nowplaying {
  var %dll = $qt($irc_nowplaying.dll)
  if ($isid) {
    if (!$1) tokenize 32 version
    if ($prop) return $dllcall(%dll, $prop, $$1, $2-)
    return $dll(%dll, $$1, $2-)
  }
  if ($1 == -s) return dll %dll $$2-
  noop $dllcall(%dll, noop, $$1, $2-)
}

; TODO: Doesn't take into account aarch64
alias -l irc_nowplaying.dll return $+($scriptdir, irc_nowplaying_x, $bits, .dll)

; Note: We have to expose this for mIRC, while AdiIRC allows us to keep it local.
alias irc_nowplaying:mediachanged {
  echo -at Note: *** Media changed *** $+([,$irc_nowplaying(title) - $irc_nowplaying(artist),])
  .signal -n irc_nowplaying media_changed
  noop $irc_nowplaying(wait_for_media, data).irc_nowplaying:mediachanged
}

alias irc_nowplaying.haltall echo -at [irc_nowplaying] Cancelled all asynchronous calls $irc_nowplaying(halt)
