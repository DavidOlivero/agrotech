import { Component } from '@angular/core';
import { CommonModule } from '@angular/common';
import { invoke } from "@tauri-apps/api/core";
import { FormControl, FormGroup, ReactiveFormsModule } from "@angular/forms";

@Component({
  selector: 'app-root',
  standalone: true,
  imports: [CommonModule, ReactiveFormsModule],
  templateUrl: './app.component.html',
})
export class AppComponent {
  public response?: String
  public form = new FormGroup({
    aiModel: new FormControl('CHATGPT'),
  })
  async askToGpt() {
    const value = this.form.controls.aiModel.value
    console.log(value)
    this.response = await invoke<string>('ask_to_ai', { aiModel: value })
  }
}
