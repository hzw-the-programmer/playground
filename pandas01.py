import pandas as pd
import matplotlib.pyplot as plt
from matplotlib import style

web_stats = {'Day': [1, 2, 3, 4, 5, 6], 'Visitors': [43, 34, 65, 56, 29, 76],
             'Bounvfce Rate': [65, 67, 78, 65, 45, 52]}
days = web_stats['Day']

df = pd.DataFrame(web_stats)
df.set_index('Day', inplace=True)

print(df)
# print(df.head())
# print(df.tail())
# print(df['Visitors'])
style.use('fivethirtyeight')
df['Visitors'].plot()
plt.show()
