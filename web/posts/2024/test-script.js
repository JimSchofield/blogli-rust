class MyCustomElement extends HTMLElement {
  connectedCallback() {
    console.lot("I exist", this);
  }
}

if (!customElements.get('my-custom-element')) {
  customElements.define('my-custom-element', MyCustomElement);
}
