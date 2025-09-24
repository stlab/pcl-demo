;;; Directory Local Variables for rust-analyzer target switching
;;; For more information see (info "(emacs) Directory Variables")

;; Current configuration: Web/WASM32 target
;; To switch targets, use ./scripts/switch-target.sh [web|desktop|ios]

((rust-mode . ((lsp-rust-analyzer-cargo-target . "wasm32-unknown-unknown")
               (lsp-rust-analyzer-check-command . "clippy")
               (lsp-rust-analyzer-cargo-watch-args . ["--target" "wasm32-unknown-unknown"])
               (lsp-rust-analyzer-cargo-all-targets . nil)
               (lsp-rust-analyzer-cargo-features . ["web"])
               (lsp-rust-analyzer-proc-macro-enable . t)
               (lsp-rust-analyzer-cargo-load-out-dirs-from-check . t))))
