require 'libui'

UI = LibUI

UI.init

main_window = UI.new_window('Miyuki X editor', 200, 700, 1)

# text box
# ranndering Viwer

# btm(post)
button = UI.new_button('Button')

# do post request rust server
UI.button_on_clicked(button) do
  UI.msg_box(main_window, 'Information', 'You clicked the button')
end

UI.window_set_child(main_window, button) # text box, vIwer box
UI.control_show(main_window) #sub widows(viwer)

UI.main
UI.quit