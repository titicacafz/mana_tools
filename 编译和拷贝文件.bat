@echo off

@echo ±àÒëlog_tool£¬ÇëµÈ´ý¡­¡­

cd ../log_tool
cargo clean
cargo build --release

@echo ±àÒëmqtt_client£¬ÇëµÈ´ý¡­¡­
cd ../mqtt_client
cargo clean
cargo build --release

@echo ±àÒëlog_viewer£¬ÇëµÈ´ý¡­¡­
cd ../log_viewer
cargo clean
cargo build --release


@echo ¿½±´¿ÉÖ´ÐÐÎÄ¼þ
xcopy "..\log_tool\target\release\log_tool.exe" ".\" /y
xcopy "..\log_tool\target\release\log_tool.exe" ".\innosetup\" /y
xcopy "..\mqtt_client\target\release\mqtt_client.exe" ".\" /y
xcopy "..\mqtt_client\target\release\mqtt_client.exe" ".\innosetup\" /y
xcopy ".\target\release\log_viewer.exe" ".\innosetup\" /y
xcopy ".\config.toml" ".\innosetup\" /y
xcopy ".\log.toml" ".\innosetup\" /y 
xcopy ".\cross_interface.txt" ".\innosetup\" /y 
xcopy ".\static" ".\innosetup\static\" /y /s 

pause