import "./MainView.scss";
import html from "./MainView.html?raw";
import { elementFromHTML, bindAttributes } from "./Util";
import Fretboard from "./Fretboard";
import NotePad from "./NotePad";
import NanJ from "./NanJ";
import type { Pitch } from "./types";
import Question from "./Question";
import type { QuestionItem } from "./Question";

export default class MainViewController {
  el: HTMLElement;
  content = {} as HTMLElement;
  fretboard: Fretboard;
  nanJ = {} as NanJ;
  question = new Question();
  questionItem = {} as QuestionItem;
  
  constructor() {
    this.el = elementFromHTML<HTMLElement>(html);
    bindAttributes(this, this.el);

    this.fretboard = new Fretboard();
    this.content.appendChild(this.fretboard.el);

    const notePad = new NotePad();
    notePad.onClick = this.onNotePadClick.bind(this);
    this.content.appendChild(notePad.el);

    this.supplyQuestion();
  }

  onNotePadClick(pitch: Pitch) {
    if (this.questionItem.pitch == pitch) {
      this.nanJ.state = "happy";
      this.supplyQuestion();
    } else {
      this.nanJ.state = "angry";
    }
  }

  onNanJFadeout(nanJ: NanJ) {
    nanJ.onFadeOut = undefined;
    nanJ.el.remove();
  }

  supplyQuestion() {
    this.questionItem = this.question.lot();

    this.nanJ = new NanJ();
    this.nanJ.el.style.setProperty("--string", this.questionItem.string.toString());
    this.nanJ.el.style.setProperty("--fret", this.questionItem.fret.toString());
    this.nanJ.onFadeOut = this.onNanJFadeout.bind(this);

    this.fretboard.el.appendChild(this.nanJ.el);
  }
}
