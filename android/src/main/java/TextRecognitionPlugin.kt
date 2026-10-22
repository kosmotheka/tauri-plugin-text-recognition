package app.kosmotheka.textrecognition

import android.app.Activity
import android.graphics.Rect
import android.net.Uri
import app.tauri.annotation.Command
import app.tauri.annotation.InvokeArg
import app.tauri.annotation.TauriPlugin
import app.tauri.plugin.Invoke
import app.tauri.plugin.JSArray
import app.tauri.plugin.JSObject
import app.tauri.plugin.Plugin
import com.google.mlkit.common.MlKitException
import com.google.mlkit.vision.common.InputImage
import com.google.mlkit.vision.text.Text
import com.google.mlkit.vision.text.TextRecognition
import com.google.mlkit.vision.text.latin.TextRecognizerOptions
import java.io.File

@InvokeArg
class RecognizeTextArgs {
    lateinit var path: String
}

@TauriPlugin
class TextRecognitionPlugin(private val activity: Activity) : Plugin(activity) {
    private val recognizer by lazy {
        TextRecognition.getClient(TextRecognizerOptions.DEFAULT_OPTIONS)
    }

    @Command
    fun recognizeText(invoke: Invoke) {
        val args = invoke.parseArgs(RecognizeTextArgs::class.java)

        val image = try {
            InputImage.fromFilePath(activity, toUri(args.path))
        } catch (e: Exception) {
            invoke.reject("Could not open the image at ${args.path}: ${e.message}")
            return
        }

        recognizer.process(image)
            .addOnSuccessListener { text -> invoke.resolve(toResult(text, image)) }
            .addOnFailureListener { e -> invoke.reject(failureMessage(e)) }
    }

    private fun toUri(path: String): Uri =
        if (path.startsWith("content://") || path.startsWith("file://")) {
            Uri.parse(path)
        } else {
            Uri.fromFile(File(path))
        }

    private fun failureMessage(e: Exception): String =
        if (e is MlKitException && e.errorCode == MlKitException.UNAVAILABLE) {
            "The text recognition model is not on this device yet. " +
                "Google Play Services downloads it; try again in a moment."
        } else {
            "Text recognition failed: ${e.message}"
        }
}

private fun toResult(text: Text, image: InputImage): JSObject {
    val sideways = image.rotationDegrees == 90 || image.rotationDegrees == 270
    val blocks = JSArray()
    for (block in text.textBlocks) {
        val lines = JSArray()
        for (line in block.lines) {
            lines.put(
                JSObject()
                    .put("text", line.text)
                    .put("bounds", toBounds(line.boundingBox))
                    .put("confidence", line.confidence.toDouble())
                    .put("angle", line.angle.toDouble())
            )
        }
        blocks.put(
            JSObject()
                .put("text", block.text)
                .put("bounds", toBounds(block.boundingBox))
                .put("lines", lines)
        )
    }

    return JSObject()
        .put("text", text.textBlocks.joinToString("\n\n") { it.text })
        .put("imageWidth", if (sideways) image.height else image.width)
        .put("imageHeight", if (sideways) image.width else image.height)
        .put("blocks", blocks)
}

private fun toBounds(rect: Rect?): JSObject? =
    rect?.let {
        JSObject()
            .put("left", it.left)
            .put("top", it.top)
            .put("width", it.width())
            .put("height", it.height())
    }
