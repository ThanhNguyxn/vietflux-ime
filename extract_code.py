import json
import os
import re

json_path = r"d:\Code\vietflux-ime\temp_ui\ai_chat.json"
output_dir = r"d:\Code\vietflux-ime\extracted_ui"

with open(json_path, 'r', encoding='utf-8') as f:
    data = json.load(f)

# Map toolCallId to path
tool_paths = {}
# Map path to content (latest)
file_contents = {}

# Iterate through all messages in all threads
for thread in data.get('threads', []):
    for msg in thread.get('messages', []):
        for part in msg.get('parts', []):
            part_type = part.get('partType')
            
            if part_type == 'tool-call-json-DO-NOT-USE-IN-PROD':
                content_json = part.get('contentJson')
                if content_json:
                    try:
                        tool_call = json.loads(content_json)
                        if tool_call.get('toolName') == 'fast_apply_tool':
                            tool_id = tool_call.get('toolCallId')
                            args_str = tool_call.get('argsJson')
                            if args_str:
                                args = json.loads(args_str)
                                path = args.get('path')
                                if tool_id and path:
                                    tool_paths[tool_id] = path
                    except Exception as e:
                        print(f"Error parsing tool call: {e}")

            elif part_type == 'tool-result-json-DO-NOT-USE-IN-PROD':
                content_json = part.get('contentJson')
                if content_json:
                    try:
                        tool_result = json.loads(content_json)
                        tool_id = tool_result.get('toolCallId')
                        result_json_str = tool_result.get('resultJson')
                        
                        if tool_id and result_json_str:
                            result_json = json.loads(result_json_str)
                            if result_json.get('success'):
                                content = result_json.get('content')
                                path = tool_paths.get(tool_id)
                                if path and content:
                                    file_contents[path] = content
                    except Exception as e:
                        print(f"Error parsing tool result: {e}")

# Write files
if not os.path.exists(output_dir):
    os.makedirs(output_dir)

for path, content in file_contents.items():
    # Remove leading slash
    rel_path = path.lstrip('/')
    full_path = os.path.join(output_dir, rel_path)
    
    # Create dirs
    os.makedirs(os.path.dirname(full_path), exist_ok=True)
    
    with open(full_path, 'w', encoding='utf-8') as f:
        f.write(content)
    print(f"Extracted: {rel_path}")
