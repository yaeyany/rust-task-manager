class PasswordInput extends HTMLElement {
    constructor() {
        super();
        
        this.innerHTML = `
            <style>
                .password-wrapper {
                    position: relative;
                    display: block;
                    width: 100%;
                }
                .password-wrapper input {
                    width: 100%;
                    padding-right: 45px;
                    box-sizing: border-box;
                }
                .password-toggle {
                    position: absolute;
                    top: 50%;
                    right: 8px;
                    width: 30px;
                    height: 30px;
                    transform: translateY(-50%);
                    display: flex;
                    align-items: center;
                    justify-content: center;
                    padding: 0;
                    border: none;
                    background: transparent;
                    cursor: pointer;
                }
                .password-toggle svg {
                    width: 20px;
                    height: 20px;
                    stroke: currentColor;
                    fill: none;
                }
            </style>
            
            <svg xmlns="http://w3.org" style="display: none;">
                <symbol id="eye-open" viewBox="0 0 24 24" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M2 12s3-7 10-7 10 7 10 7-3 7-10 7-10-7-10-7z"/>
                    <circle cx="12" cy="12" r="3"/>
                </symbol>
                <symbol id="eye-closed" viewBox="0 0 24 24" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                    <path d="M9.88 9.88a3 3 0 1 0 4.24 4.24"/>
                    <path d="M10.73 5.08A10.43 10.43 0 0 1 12 5c7 0 10 7 10 7a13.16 13.16 0 0 1-1.67 2.68"/>
                    <path d="M6.61 6.61A13.52 13.52 0 0 0 2 12s3 7 10 7a9.74 9.74 0 0 0 5.39-1.61"/>
                    <line x1="2" y1="2" x2="22" y2="22"/>
                </symbol>
            </svg>

            <div class="password-wrapper">
                <input type="password" class="internal-input">
                <button type="button" class="password-toggle" aria-label="Show password">
                    <svg aria-hidden="true"><use href="#eye-closed"></use></svg>
                </button>
            </div>
        `;

        this.input = this.querySelector('.internal-input');
        this.toggleBtn = this.querySelector('.password-toggle');
        this.iconUse = this.querySelector('use');
    }

    connectedCallback() {
        const watchedAttributes = ['id', 'name', 'minlength', 'required', 'autocomplete', 'placeholder'];
        watchedAttributes.forEach(attr => {
            if (this.hasAttribute(attr)) {
                this.input.setAttribute(attr, this.getAttribute(attr));
                if (attr === 'id') this.removeAttribute('id'); 
            }
        });

        this.toggleBtn.addEventListener('click', () => {
            const isPassword = this.input.type === 'password';
            this.input.type = isPassword ? 'text' : 'password';
            this.iconUse.setAttribute('href', isPassword ? '#eye-open' : '#eye-closed');
            this.toggleBtn.setAttribute('aria-label', isPassword ? 'Hide password' : 'Show password');
        });
    }

    get value() { return this.input.value; }
    set value(val) { this.input.value = val; }
}

customElements.define('password-input', PasswordInput);
