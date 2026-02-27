package dev.helium.viewer

import android.os.Bundle
import android.widget.TextView
import androidx.appcompat.app.AppCompatActivity

class MainActivity : AppCompatActivity() {
    external fun pingFromRust(): Int

    override fun onCreate(savedInstanceState: Bundle?) {
        super.onCreate(savedInstanceState)
        setContentView(R.layout.activity_main)

        val pingValue = pingFromRust()
        findViewById<TextView>(R.id.statusText).text =
            "Rust library linked successfully (ping=$pingValue)"
    }

    companion object {
        init {
            System.loadLibrary("helium")
        }
    }
}

