package main

import (
    "fmt"
    "github.com/saintfish/chardet"
)

func main() {
    // 示例字节序列，你可以替换成从文件中读取的[]byte
    // data := []byte("这是一段测试文本。")
    // data := []byte(`//���� gbk ���뷽ʽ���Ƿ�����
    // // ����������ʾ
    // // ������`)
    data := []byte(``)

    // 使用chardet检测编码
    detector := chardet.NewTextDetector()
    result, err := detector.DetectBest(data)
    if err != nil {
        fmt.Println("Error detecting charset:", err)
        return
    }

    if result != nil {
        fmt.Printf("Detected charset: %s, Language: %s\n", result.Charset, result.Language)
    } else {
        fmt.Println("No charset detected.")
    }
}

func binarySearch(arr []int, target int) bool {
	low := 0
	high := len(arr) - 1
	for low <= high {
		mid := low + (high-low)/2
		if arr[mid] == target {
			return true
		} else if arr[mid] < target {
			low = mid + 1
		} else {
			high = mid - 1
		}
	}
	return false
}
