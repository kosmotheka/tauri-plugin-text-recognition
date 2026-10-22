package app.kosmotheka.textrecognition

import android.app.Activity
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin

@InvokeArg
class RecognizeTextArgs {
    lateinit var path: String
}

@TauriPlugin
class TextRecognitionPlugin(private val activity: Activity) : Plugin(activity) {
    @Command
    fun recognizeText(invoke: Invoke) {
        invoke.parseArgs(RecognizeTextArgs::class.java)

        // Milestone 2 replaces this empty result with ML Kit's.
        val result = JSObject()
        result.put("text", "")
        result.put("imageWidth", 0)
        result.put("imageHeight", 0)
        result.put("blocks", JSArray())
        invoke.resolve(result)
    }
}
