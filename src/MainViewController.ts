import "./MainView.scss";
import html from "./MainView.html?raw";
import { elementFromHTML, bindAttributes } from "./Util";
import Fretboard from "./Fretboard";
import NotePad from "./NotePad";
import NanJ from "./NanJ";
import type { Pitch } from "./types";

export default class MainViewController {
  el: HTMLElement;
  content = {} as HTMLElement;
  fretboard: Fretboard;
  nanJ: NanJ;
  
  constructor() {
    this.el = elementFromHTML<HTMLElement>(html);
    bindAttributes(this, this.el);

    this.fretboard = new Fretboard();
    this.content.appendChild(this.fretboard.el);

    const notePad = new NotePad();
    notePad.onClick = this.onNotePadClick.bind(this);
    this.content.appendChild(notePad.el);

    this.nanJ = new NanJ();
    this.nanJ.onFadeOut = this.onNanJFadeout.bind(this);
    this.fretboard.el.appendChild(this.nanJ.el);
  }

  onNotePadClick(pitch: Pitch) {
    switch (this.nanJ.state) {
    case "normal":
      this.nanJ.state = "angry";
      break;
    case "angry":
      this.nanJ.state = "happy";
      break;
    case "happy":
      this.nanJ.state = "normal";
      break;
    }
  }

  onNanJFadeout(nanJ: NanJ) {
    nanJ.el.remove();
  }
}
