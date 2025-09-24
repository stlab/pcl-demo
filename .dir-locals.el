;;; Directory Local Variables for rust-analyzer target switching
;;; For more information see (info "(emacs) Directory Variables")

;; Default configuration: Web/WASM32 target
;; To switch targets, use ./scripts/switch-target.sh [web|desktop|ios]

((rust-mode . ((lsp-rust-analyzer-cargo-target . "wasm32-unknown-unknown")
               (lsp-rust-analyzer-check-command . "clippy")
               (lsp-rust-analyzer-cargo-watch-args . ["--target" "wasm32-unknown-unknown"])
               (lsp-rust-analyzer-cargo-all-targets . nil)
               (lsp-rust-analyzer-cargo-features . ["web"])
               (lsp-rust-analyzer-proc-macro-enable . t)
               (lsp-rust-analyzer-cargo-load-out-dirs-from-check . t))))

;; Alternative configurations (uncomment the desired target):

;; DESKTOP PROFILE (native target)
;; ((rust-mode . ((lsp-rust-analyzer-cargo-target . nil)
;;                (lsp-rust-analyzer-check-command . "clippy")
;;                (lsp-rust-analyzer-cargo-watch-args . [])
;;                (lsp-rust-analyzer-cargo-all-targets . t)
;;                (lsp-rust-analyzer-cargo-features . [])
;;                (lsp-rust-analyzer-proc-macro-enable . t)
;;                (lsp-rust-analyzer-cargo-load-out-dirs-from-check . t))))

;; IOS PROFILE
;; ((rust-mode . ((lsp-rust-analyzer-cargo-target . "aarch64-apple-ios-sim")
;;                (lsp-rust-analyzer-check-command . "clippy")
;;                (lsp-rust-analyzer-cargo-watch-args . ["--target" "aarch64-apple-ios-sim"])
;;                (lsp-rust-analyzer-cargo-all-targets . nil)
;;                (lsp-rust-analyzer-cargo-features . ["mobile"])
;;                (lsp-rust-analyzer-proc-macro-enable . t)
;;                (lsp-rust-analyzer-cargo-load-out-dirs-from-check . t))))
