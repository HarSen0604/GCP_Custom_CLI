from flask import Flask, request, jsonify
from flask_cors import CORS
import google.generativeai as genai
import os
import re

app = Flask(__name__)
CORS(app)  # Enable CORS for frontend communication

# Set your Gemini API key
genai.configure(api_key="AIzaSyBUmapZZX9f5dC0darbYQUrHWra7lUJ_oM")

# Suppress Google Cloud SDK warnings
os.environ["CLOUDSDK_CORE_DISABLE_PROMPTS"] = "1"
os.environ["CLOUDSDK_SUPPRESS_RUNTIME_WARNINGS"] = "1"
os.environ["CLOUDSDK_METRICS_ENVIRONMENT"] = "NONE"
os.environ["CLOUDSDK_CORE_LOG_LEVEL"] = "ERROR"

def nlp_to_gcloud_command(nlp_input):
    """Convert NLP input to a Google Cloud CLI command using Gemini API."""
    prompt = (
        "Convert the following natural language request into a valid Google Cloud CLI command. "
        "If no exact match is found, provide the most relevant command. "
        "Only return the command without any markdown formatting, explanations, or extra symbols.\n\n"
        f"Request: '{nlp_input}'\nCommand:"
    )

    response = genai.GenerativeModel("gemini-2.0-flash").generate_content(prompt)

    # Extract and clean the command
    command = response.text.strip().replace("```bash", "").replace("```", "").strip()

    # Ensure output is only the command
    return command if command.startswith("gcloud") else "No valid Google Cloud command found."

@app.route("/process_text", methods=["POST"])
def process_text():
    """API endpoint to process user input and return the GCloud command."""
    data = request.json
    user_text = data.get("text", "")

    if not user_text:
        return jsonify({"error": "No input provided"}), 400

    gcloud_command = nlp_to_gcloud_command(user_text)
    return jsonify({"response": gcloud_command})

if __name__ == "__main__":
    app.run(debug=True, port=5000)
