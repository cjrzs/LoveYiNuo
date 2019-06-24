import shutil
import os

dirs = "E:\测试图片"
list_file = os.listdir(dirs)
print(list_file)
i = 1
for dir1 in list_file:
    #取到文件后缀
    file_jpg = os.path.splitext(dir1)[1]
    print(type(file_jpg))
    if file_jpg == '.jpg':
        new_filename = str(i)+'.jpg'
        i+=1
        new_file = os.path.join('E:\测试图片2',new_filename)
        #os.rename可以只能移动同盘符下的文件或者文件夹
        #要移动不同盘符下的文件或者文件夹必须用shutil.move
        os.rename(os.path.join(dirs,dir1),new_file)
        # shutil.move(dir1, new_filename)








