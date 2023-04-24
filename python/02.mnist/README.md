# Hands-on #2: Deep learning with AWS

- [Hands-on #2: Deep learning with AWS](https://tomomano.github.io/learn-aws-by-coding/#sec_jupyter_and_deep_learning)

![app_arch](./figs/handson-02-architecture.png "App architecture")

| Instance     | GPUs | GPU model   | GPU Mem(GiB) | vCPU | Mem(GiB) | Price per hour($) |
| :----------- | :--- | :---------- | :----------- | :--- | :------- | :---------------- |
| p3.2xlarge   | 1    | NVIDIA V100 | 16           | 8    | 61       | 3.06              |
| p3n.16xlarge | 8    | NVIDIA V100 | 128          | 64   | 488      | 24.48             |
| p2.xlarge    | 1    | NVIDIA K80  | 12           | 4    | 61       | 0.9               |
| g4dn.xlarge  | 1    | NVIDIA T4   | 16           | 4    | 16       | 0.526             |

## DLAMI(Deep Learning Amazon Machine Image)
AMI(Amazon Machine Image)とは大まかにOSに相当するが、加えて各種のプログラムがインストール済みのAMIも定義することができる。
必要なプログラムがインストールされているAMIを使うことで環境構築が楽になる。(Dockerでいうベースイメージ)
ディープラーニングで使われるプログラムが事前にインストールしてあるAMIがDLAMI(Deep Learning AMI)である。
DLAMIには`TensorFlow`や`PyTorch`などのディープラーニングのフレームワーク・ライブラリがインストールされている。
このハンズオンでは、Amazon Linux 2をベースにしたDLAMI(AMI ID: ap-northeast-1=ami-09c0c16fc46a29ed9, us-east-1=ami-060f07284bb6f9faf)を使用。
※AMIのIDはリージョンごとで違うので注意

## NOTION
- 初期状態の AWS アカウントでは， GPU 搭載の Gタイプのインスタンスの起動上限が0になっていることがある． これを確認するには， AWS コンソールから EC2 の画面を開き，左のメニューから Limits を選択する． その中の Running On-Demand All G instances という数字が G インスタンスの起動上限を表している．
もし，これが 0 になっていた場合は， AWS の自動申請フォームから上限緩和のリクエストを送る必要がある． 詳しくは 公式ドキュメンテーション ["Amazon EC2 service quotas"](https://docs.aws.amazon.com/AWSEC2/latest/UserGuide/ec2-resource-limits.html) を参照のこと．

- このハンズオンは， g4dn.xlarge タイプの EC2 インスタンスを使うので，東京 (ap-northeast-1) リージョンでは 0.71 $/hour のコストが発生する．

- AWS Educate Starter Account を使用している読者へ: 執筆時点においては， Starter Account には GPU 搭載型インスタンスを起動できないという制限が設けられている． したがって， Starter Account のユーザーはこのハンズオンを実行することはできない． 興味のある読者は，制限のない一般アカウントを自分自身で取得する必要があることに注意．