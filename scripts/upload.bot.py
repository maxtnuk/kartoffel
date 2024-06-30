import requests

down_under = "8fed-b281-bb40-9276"
total_mayhem = "7890-40a1-1c32-3815"

base_url="https://kartoffels.pwy.io"
bot_path="target/riscv64-kartoffel-bot/release/kartoffel"
current_bot="08f3-de73-68f4-304e"

def upload_bot(room):
    with open(bot_path, 'rb') as file:
        bot_data= file.read()
        response = requests.post(f"{base_url}/api/worlds/{room}/bots",data=bot_data)
        if response.status_code == 200:
            print('파일 업로드 성공!')
            print('서버 응답:')
            bot_response = response.json()
            print(bot_response)
        else:
            print(f'파일 업로드 실패. 응답 코드: {response.status_code}')
    
if __name__ == "__main__":
    upload_bot(down_under)