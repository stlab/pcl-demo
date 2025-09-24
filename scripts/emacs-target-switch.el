;;; emacs-target-switch.el --- Interactive target switching for Emacs -*- lexical-binding: t; -*-

;; Author: Auto-generated
;; Version: 1.0
;; Package-Requires: ((emacs "25.1"))

;;; Commentary:

;; This file provides interactive functions for switching rust-analyzer targets
;; in Emacs. It integrates with the shell scripts to provide a seamless
;; target switching experience.

;;; Code:

(defvar rust-target-switch-script-path
  (expand-file-name "scripts/switch-target.sh"
                    (locate-dominating-file default-directory ".dir-locals.el"))
  "Path to the target switching script.")

(defun rust-target-switch-web ()
  "Switch rust-analyzer target to wasm32-unknown-unknown for web development."
  (interactive)
  (rust-target-switch-to "web"))

(defun rust-target-switch-desktop ()
  "Switch rust-analyzer target to native for desktop development."
  (interactive)
  (rust-target-switch-to "desktop"))

(defun rust-target-switch-ios ()
  "Switch rust-analyzer target to aarch64-apple-ios-sim for iOS development."
  (interactive)
  (rust-target-switch-to "ios"))

(defun rust-target-switch-to (target)
  "Switch rust-analyzer target to TARGET and reload LSP workspace."
  (let ((script-path rust-target-switch-script-path))
    (if (and script-path (file-exists-p script-path))
        (progn
          (message "Switching to %s target..." target)
          (let ((result (shell-command-to-string 
                        (format "%s %s --ide=emacs" script-path target))))
            (message "%s" (string-trim result))
            ;; Reload dir-locals and restart LSP
            (hack-dir-local-variables-non-file-buffer)
            (when (fboundp 'lsp-restart-workspace)
              (lsp-restart-workspace))
            (message "Switched to %s target and reloaded LSP" target)))
      (error "Target switching script not found at %s" script-path))))

(defun rust-target-show-current ()
  "Show the current rust-analyzer target configuration."
  (interactive)
  (let ((script-path (expand-file-name "scripts/show-target.sh"
                                       (locate-dominating-file default-directory ".dir-locals.el"))))
    (if (and script-path (file-exists-p script-path))
        (let ((result (shell-command-to-string script-path)))
          (message "%s" (string-trim result)))
      (error "Show target script not found"))))

;; Optional: Add to mode hook for easy access
;;;###autoload
(defun rust-target-switch-setup-keybindings ()
  "Set up convenient keybindings for target switching in rust-mode."
  (local-set-key (kbd "C-c t w") #'rust-target-switch-web)
  (local-set-key (kbd "C-c t d") #'rust-target-switch-desktop)
  (local-set-key (kbd "C-c t i") #'rust-target-switch-ios)
  (local-set-key (kbd "C-c t s") #'rust-target-show-current))

;; Uncomment the following line to automatically set up keybindings in rust-mode
;; (add-hook 'rust-mode-hook #'rust-target-switch-setup-keybindings)

(provide 'emacs-target-switch)

;;; emacs-target-switch.el ends here
