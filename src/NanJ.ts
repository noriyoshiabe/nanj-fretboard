import "./NanJ.scss";

export type NanJState = "normal" | "happy" | "angry";

export default class NanJ {
  el: HTMLElement;
  _state: NanJState = "normal";
  onFadeOut?: (nanJ: NanJ) => void;

  constructor() {
    this.el = document.createElement("div") as HTMLElement;
    this.el.classList.add("NanJ");

    const span = document.createElement("span") as HTMLElement;
    this.el.appendChild(span);

    this.el.classList.add("--" + this._state);

    this.el.addEventListener("animationend", () => {
      if (this._state == "happy") {
        this.onFadeOut?.(this);
      }
    });
  }

  set state(state: NanJState) {
    const needDelay = this._state == state;
    this._state = state;
    this.el.classList.remove("--normal");
    this.el.classList.remove("--angry");
    this.el.classList.remove("--happy");

    if (needDelay) {
      setTimeout(() => this.el.classList.add("--" + state));
    } else {
      this.el.classList.add("--" + state);
    }
  }

  get state() {
    return this._state;
  }
}
