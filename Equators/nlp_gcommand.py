from flask import Flask, request, jsonify
from flask_cors import CORS
import google.generativeai as genai
import os
import json
import argparse

app = Flask(__name__)
CORS(app)  # Enable CORS for frontend communication

# Set your Gemini API key
genai.configure(api_key="AIzaSyBUmapZZX9f5dC0darbYQUrHWra7lUJ_oM")

# Suppress Google Cloud SDK warnings
os.environ["CLOUDSDK_CORE_DISABLE_PROMPTS"] = "1"
os.environ["CLOUDSDK_SUPPRESS_RUNTIME_WARNINGS"] = "1"
os.environ["CLOUDSDK_METRICS_ENVIRONMENT"] = "NONE"
os.environ["CLOUDSDK_CORE_LOG_LEVEL"] = "ERROR"

# Argument parser to determine the JSON file
def get_json_prompt():
    parser = argparse.ArgumentParser()
    parser.add_argument("--type", choices=["new", "experienced"], required=True)
    args, _ = parser.parse_known_args()
    
    json_file = "newDevPrompt.json" if args.type == "new" else "expdevprompt.json"
    
    try:
        with open(json_file, "r") as file:
            json_data = json.load(file)
        return json.dumps(json_data)
    except Exception as e:
        return "{}"  # Return an empty JSON object if file read fails

def nlp_to_gcloud_command(nlp_input):
    """Convert NLP input to a Google Cloud CLI command using Gemini API. Auto-correct the input if there's a close match."""
    json_prompt = get_json_prompt()
   
    prompt = (
        "Convert the following natural language request into a valid Google Cloud CLI command. "
        "If there are spelling mistakes, correct them before generating the command. "
        "If no exact match is found, provide the most relevant command. "
        "Only return the command without any markdown formatting, explanations, or extra symbols.\n\n"
        f"Request: '{nlp_input}'\nCommand:"
    )
    

    response = genai.GenerativeModel("gemini-2.0-flash").generate_content(prompt)
    
    # Extract and clean the command
    command = response.text.strip().replace("bash", "").replace("", "").strip()
    print("Command:",command)
    # Ensure output is only the command
    return command if command.startswith("gcloud") else "No valid Google Cloud command found."
def autocorrect_gcloud_command(command_input):
    prompt = (
        "Correct the following Google Cloud CLI command to its proper format. "
        "Only return the corrected command without any markdown formatting, explanations, or extra symbols.\n\n"
        f"Command: '{command_input}'\nCorrected Command:"
    )
    
    response = genai.GenerativeModel("gemini-2.0-flash").generate_content(prompt)
    corrected_command = response.text.strip().replace("```bash", "").replace("```", "").strip()
    return corrected_command if corrected_command.startswith("gcloud") else command_input

def generate_command(nlp_input, prompt_file):
    """Generate both the full and short form of a command by searching in the given JSON prompt file."""
    try:
        with open(prompt_file, "r") as file:
            prompt_data = json.load(file)

        # Search for the command in the JSON structure
        for category, details in prompt_data.items():
            if isinstance(details, dict) and "commands" in details:
                for command in details["commands"]:
                    if isinstance(command, dict):
                        short_command = command.get("short", "").lower()
                        full_command = command.get("full", "").lower()
                        if nlp_input.lower() in short_command or nlp_input.lower() in full_command:
                            return {
                                "full_command": command.get("full", "No matching command found."),
                                "short_command": command.get("short", "No short form available.")
                            }

            elif isinstance(details, dict):
                for description, full_command in details.items():
                    if isinstance(description, str) and isinstance(full_command, str):
                        if nlp_input.lower() in description.lower():
                            return {
                                "full_command": full_command,
                                "short_command": "No short form available."  # If short form is not defined in JSON
                            }

        return {
            "full_command": "No matching command found.",
            "short_command": "No short form available."
        }

    except Exception as e:
        return {
            "error": f"Error processing JSON file: {str(e)}"
        }

@app.route("/process_text", methods=["POST"])
def process_text():
    """API endpoint to process user input and return both full and short GCloud commands."""
    data = request.json
    user_text = data.get("text", "").strip()

    if not user_text:
        return jsonify({"error": "No input provided"}), 400

    # First, try to autocorrect the command
    corrected_command = autocorrect_gcloud_command(user_text)

    # If the corrected command is different from the input and starts with 'gcloud', return the corrected version
    if corrected_command != user_text and corrected_command.startswith("gcloud"):
        gcloud_command=corrected_command
    else:
    # Otherwise, convert NLP input into a GCloud command
        gcloud_command = nlp_to_gcloud_command(user_text)

    # Determine the correct JSON file
    json_file = "newDevPrompt.json" if "new" in user_text.lower() else "expdevprompt.json"

    # Generate commands based on the converted GCloud command
    command_output = generate_command(gcloud_command, json_file)
    full_command = command_output.get("full_command", "No full command found.")
    short_command = command_output.get("short_command", "No short command found.")

    return jsonify({"response": [f"Full: {full_command}", f"[ Short: {short_command} ]"]})


if __name__ == "__main__":
    app.run(debug=True, port=5000)
