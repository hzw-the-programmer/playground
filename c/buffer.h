typedef struct buffer {
    size_t size;
    size_t write;
    size_t read;
    uint8_t *data;
} buffer_t;

extern buffer_t* createBuffer(size_t size);
extern void destroyBuffer(buffer_t *buf);

extern uint8_t* getBufferWritePtr(buffer_t *buf);
extern size_t _getBufferWriteLen(buffer_t *buf);
extern void advanceBufferWrite(buffer_t *buf, size_t len);
extern size_t getBufferWriteLen(buffer_t *buf);
extern size_t writeToBuffer(buffer_t *buf, uint8_t *data, size_t len);

extern uint8_t* getBufferReadPtr(buffer_t *buf);
extern size_t _getBufferReadLen(buffer_t *buf);
extern void advanceBufferRead(buffer_t *buf, size_t len);
extern size_t getBufferReadLen(buffer_t *buf);
extern size_t readFromBuffer(buffer_t *buf, uint8_t *data, size_t len);

extern void deAdvanceBufferRead(buffer_t *buf, size_t len);
