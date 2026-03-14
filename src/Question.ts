import { shuffle } from "./Util";
import type { Pitch } from "./types";
import { PitchList } from "./types";

export type QuestionItem = {
  string: number;
  fret: number;
  pitch: Pitch;
};

export default class Question {
  items: QuestionItem[];
  lotItems: QuestionItem[];

  constructor() {
    this.items = [];

    for (let i = 1; i <= 6; i++) {
      let pitchIndex = 0;

      switch (i) {
      case 1:
        pitchIndex = PitchList.indexOf("E");
        break;
      case 2:
        pitchIndex = PitchList.indexOf("B");
        break;
      case 3:
        pitchIndex = PitchList.indexOf("G");
        break;
      case 4:
        pitchIndex = PitchList.indexOf("D");
        break;
      case 5:
        pitchIndex = PitchList.indexOf("A");
        break;
      case 6:
        pitchIndex = PitchList.indexOf("E");
        break;
      }

      for (let j = 0; j <= 12; j++) {
        this.items.push({string: i, fret: j, pitch: PitchList[pitchIndex]});
        pitchIndex++;
        if (PitchList.length <= pitchIndex) {
          pitchIndex = 0;
        }
      }
    }

    this.lotItems = shuffle(this.items);
  }

  lot(): QuestionItem {
    if (!this.lotItems.length) {
      this.lotItems = shuffle(this.items); 
    }
    return this.lotItems.shift()!;
  }
}
